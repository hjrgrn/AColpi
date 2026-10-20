use std::fmt::Debug;

use aes_gcm::{AesGcm, aes::Aes256};
use hmac::digest::{
    consts::{B0, B1},
    typenum::{UInt, UTerm},
};
use tokio::io::{self, AsyncRead, AsyncWrite};

use crate::shared::utils::error_chain_fmt;

pub const CIPTEXT_W_NONCE_SIZE: usize = 1052;
pub const CIPTEXT_W_NONCE_SIZE_AND_HASH: usize = 1084;

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
        // TODOFIRST:
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
    _reader: T,
    s_tools: SafetyTools,
}

impl<T: AsyncRead + Unpin + Send> ReadHandler<T> {
    pub fn new(_reader: T) -> Self {
        // TODO:
        Self {
            _reader,
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
        // TODOFIRST:
        Err(ReadHandlerError::WIP)
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
}

enum SafetyTools {
    Active {
        aes_cipher: AesGcm<Aes256, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>,
        hmac_key: [u8; 32],
        seq_number: u32,
        buffer_size: usize,
    },
    Inactive {
        buffer_size: usize,
    },
}

impl SafetyTools {
    pub fn new() -> Self {
        Self::Inactive {
            buffer_size: CIPTEXT_W_NONCE_SIZE,
        }
    }
    pub fn buffer_size(&self) -> usize {
        match self {
            Self::Active {
                aes_cipher: _,
                hmac_key: _,
                seq_number: _,
                buffer_size,
            } => *buffer_size,
            Self::Inactive { buffer_size } => *buffer_size,
        }
    }
}
