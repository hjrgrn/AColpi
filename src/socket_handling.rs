use std::fmt::Debug;

use aes_gcm::{AesGcm, Nonce, aead::Aead, aes::Aes256};
use anyhow::Context;
use hmac::{
    Hmac, Mac,
    digest::{
        KeyInit,
        consts::{B0, B1},
        typenum::{UInt, UTerm},
    },
};
use sha2::Sha256;
use tokio::io::{self, AsyncRead, AsyncReadExt, AsyncWrite};

use crate::shared::utils::error_chain_fmt;

type AG256 = AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

// TODO: Packet size will have to be configurable. Maybe the whole handlers, maybe they should be
// traits.
//
/// Size of the ciphertext block in bytes.
pub const CIP_TEXT_SIZE: usize = 1040;
/// Size of the block composed of the ciphertext + the nonce.
pub const CIP_TEXT_W_NONCE_SIZE: usize = 1052;
/// Size of the block composed of the ciphertext + the nonce + hash.
pub const CIP_TEXT_W_NONCE_SIZE_AND_HASH: usize = 1084;
/// Size of the plaintext block.
pub const PLAIN_PACKET_SIZE: usize = 1024;
/// XXX:
pub const P_PACKET_PREFIX_SIZE: usize = 4;
/// XXX:
pub const SEQ_NUM_SIZE: usize = 10;

#[derive(thiserror::Error)]
pub enum WriteHandlerError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    MalformedPacket(#[from] anyhow::Error),
    // TODO: something better
    #[error(transparent)]
    EncryptionError(anyhow::Error),
    // TODO:
    // #[error(transparent)]
    // HmacError(#[from] InvalidLength),
    #[error("WIP")]
    WIP,
}

impl Debug for WriteHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// # `WriteHandler`
///
/// This struct handles the writing side of a socket.
/// It takes care also of encryption and message integrity.
/// It acts differently based on the presence of a `cipher`
/// in its fields.
// XXX: comment.
// IDEA: this could be a state machine, based on the fact that is using encryption or not.
pub struct WriteHandler<T: AsyncWrite + Unpin + Send> {
    _writer: T,
}

impl<T: AsyncWrite + Unpin + Send> WriteHandler<T> {
    pub fn new(_writer: T) -> Self {
        // TODO:
        WriteHandler { _writer }
    }

    /// # `write_str`
    ///
    /// Write a string to the socket being handled.
    /// If a cipher has been set this method encrypts the message before
    /// sending it, plaintext and ciphertext have different sizes.
    pub async fn write_str(&mut self, msg: &str) -> Result<(), WriteHandlerError> {
        let msg = msg.as_bytes();
        // FROMHERE:
        Err(WriteHandlerError::WIP)
    }
}

#[derive(thiserror::Error)]
pub enum ReadHandlerError {
    #[error("Peer closed the connection.")]
    ConnectionInterrupted,
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    MalformedPacket(#[from] anyhow::Error),
    // TODO: something better
    #[error(transparent)]
    EncryptionError(anyhow::Error),
    #[error(transparent)]
    HmacError(anyhow::Error),
    #[error("WIP")]
    WIP,
}

impl Debug for ReadHandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

/// # `ReadHandler`
///
/// This struct handles the receiving side of a socket
/// It takes care also of encryption and message integrity.
/// It acts differently based on the presence of a `cipher`
/// in its fields.
// XXX: comment
// IDEA: this could be a state machine, based on the fact that is using encryption or not. See
// above.
pub struct ReadHandler<T: AsyncRead + Unpin + Send> {
    /// Buffer used to store an incoming packet.
    buffer: [u8; CIP_TEXT_W_NONCE_SIZE_AND_HASH],
    /// Cursor used to extract an incoming packet from the stream.
    cursor: usize,
    /// Stream reader.
    reader: T,
    /// Safety tool suite.
    s_tools: SafetyTools,
}

impl<T: AsyncRead + Unpin + Send> ReadHandler<T> {
    pub fn new(reader: T) -> Self {
        // TODO:
        Self {
            buffer: [0; CIP_TEXT_W_NONCE_SIZE_AND_HASH],
            cursor: 0,
            reader,
            s_tools: SafetyTools::new(),
        }
    }

    /// # `recv_str`
    ///
    /// Receives a string from the socket being handled.
    /// If a cipher has been set this method expects to receive a ciphertext,
    /// a plaintext otherwise, those two have different sizes.
    /// The message received is assigned to the reference of String received by
    /// the caller.
    pub async fn recv_str(&mut self, line: &mut String) -> Result<(), ReadHandlerError> {
        line.clear();
        self.receive_packet().await?;
        self.decipher_packet()?;
        let size = self.validate_packet()?;
        // TODO: we may make it more efficient.
        let l = str::from_utf8(&self.buffer[self.cursor..self.cursor + size])
            .context("Line sent contained invalid UTF-8")?;
        *line = l.to_string();
        // This check is not necessary.
        if size != line.len() {
            return Err(ReadHandlerError::MalformedPacket(anyhow::anyhow!(
                "Size prefix and lenght of the line are different. This should not happen since the size of the string has been checked and `str::from_utf8` fails if there is an invalid UTF-8 char, even if the first part of the block is a perfectly valid string."
            )));
        }

        Ok(())
    }

    // `recv_bytes`
    //
    // Same as `recv_str` but it returns a vector of bytes on success.
    // NOTE: For now the encryption is not consider here because this method
    // is used only once during the handshake, when the cipher is not
    // defined yet.
    pub async fn recv_bytes(&mut self) -> Result<Vec<u8>, ReadHandlerError> {
        // TODO:
        Err(ReadHandlerError::WIP)
    }

    /// # `recv_str`'s helper function.
    ///
    /// Receives the packet from the stream, stores it in [`Self::buffer`].
    async fn receive_packet(&mut self) -> Result<(), ReadHandlerError> {
        // Set up the cursor, decide the size, based on the presence of a
        // [`SafetyTools`] suite.
        self.cursor = 0;
        let buffer_size = self.s_tools.buffer_size();

        // Receive packet.
        while self.cursor < buffer_size {
            match self.reader.read(&mut self.buffer[self.cursor..]).await {
                Ok(0) => {
                    return Err(ReadHandlerError::ConnectionInterrupted);
                }
                Ok(x) => {
                    self.cursor = self.cursor + x;
                }
                Err(e) => {
                    return Err(ReadHandlerError::IoError(e));
                }
            }
        }
        Ok(())
    }

    /// # `recv_str`'s helper function
    fn decipher_packet(&mut self) -> Result<(), ReadHandlerError> {
        match &self.s_tools {
            SafetyTools::Active {
                aes_gcm_cipher: aes_cipher,
                hmac_key,
                seq_number: _,
                buffer_size: _,
            } => {
                // Verify the bloc `ciphertext + nonce` using the provided hash.
                let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(hmac_key)
                    .map_err(|e| ReadHandlerError::HmacError(e.into()))?;
                hmac.update(&self.buffer[..CIP_TEXT_W_NONCE_SIZE]);
                hmac.verify_slice(&self.buffer[CIP_TEXT_W_NONCE_SIZE..])
                    .map_err(|e| ReadHandlerError::HmacError(e.into()))?;

                // Extract nonce.
                let nonce: &[u8; CIP_TEXT_W_NONCE_SIZE - CIP_TEXT_SIZE] = &self.buffer
                    [CIP_TEXT_SIZE..CIP_TEXT_W_NONCE_SIZE]
                    .try_into()
                    .unwrap();
                let nonce = Nonce::from(*nonce);

                // Decrypt the ciphertext.
                let buffer = aes_cipher
                    .decrypt(&nonce, &self.buffer[..CIP_TEXT_SIZE])
                    .map_err(|e| ReadHandlerError::EncryptionError(anyhow::anyhow!(e)))?;

                // Store the plaintext in the internal buffer.
                for i in 0..PLAIN_PACKET_SIZE {
                    self.buffer[i] = buffer[i];
                }
            }
            SafetyTools::Inactive { buffer_size: _ } => {}
        }

        Ok(())
    }

    /// [`recv_str`] helper method.
    ///
    /// Extract the size (and the seq number, if [`SafetyTools::Active`]) from the plaintext block.
    /// Validate the plaintext block against the expected sequence number, if [`SafetyTools::Active`].
    /// Check if the size of the packet is out of bound.
    /// Returns the size of the packet.
    fn validate_packet(&mut self) -> Result<usize, ReadHandlerError> {
        // Extract seq number.
        self.cursor = P_PACKET_PREFIX_SIZE;
        let size: usize = str::from_utf8(&self.buffer[0..self.cursor])
            .context("Malformed size prefix")?
            .parse()
            .context("Malformed size prefix")?;
        match &mut self.s_tools {
            SafetyTools::Active {
                aes_gcm_cipher: _,
                hmac_key: _,
                seq_number,
                buffer_size: _,
            } => {
                // Extract and verifies seq number.
                let recv_sec_num: u32 =
                    str::from_utf8(&self.buffer[self.cursor..self.cursor + SEQ_NUM_SIZE])
                        .context("Malformed seq number")?
                        .parse()
                        .context("Malformed seq number")?;
                if recv_sec_num != *seq_number {
                    return Err(ReadHandlerError::MalformedPacket(anyhow::anyhow!(
                        "Expecting a different seq number"
                    )));
                }
                // Update seq num and cursor.
                *seq_number = seq_number.wrapping_add(1);
                self.cursor = self.cursor + SEQ_NUM_SIZE;
            }
            SafetyTools::Inactive { buffer_size: _ } => {}
        }
        // Verifies packet size.
        if size > PLAIN_PACKET_SIZE - self.cursor {
            return Err(ReadHandlerError::MalformedPacket(anyhow::anyhow!(
                "Size of the packet out of bound"
            )));
        }
        Ok(size)
    }
}

/// XXX:
enum SafetyTools {
    /// XXX: when active provides encryption, message integrity, message authentication, reply
    /// attack protection.
    Active {
        aes_gcm_cipher: AG256,
        hmac_key: [u8; 32],
        seq_number: u32,
        buffer_size: usize,
    },
    /// XXX:
    Inactive { buffer_size: usize },
}

impl SafetyTools {
    pub fn new() -> Self {
        Self::Inactive {
            buffer_size: CIP_TEXT_W_NONCE_SIZE,
        }
    }
    pub fn buffer_size(&self) -> usize {
        match self {
            Self::Active {
                aes_gcm_cipher: _,
                hmac_key: _,
                seq_number: _,
                buffer_size,
            } => *buffer_size,
            Self::Inactive { buffer_size } => *buffer_size,
        }
    }
}
