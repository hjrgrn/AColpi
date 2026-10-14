use std::fmt::Debug;

use tokio::io::{self, AsyncRead, AsyncWrite};

use crate::shared::utils::error_chain_fmt;

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
}

impl<T: AsyncRead + Unpin + Send> ReadHandler<T> {
    pub fn new(_reader: T) -> Self {
        // TODO:
        Self { _reader }
    }

    /// # `recv_str`
    ///
    /// Receives a string from the socket being handled.
    /// If a cipher has been set this method expects to receive a ciphertext,
    /// a plaintext otherwise, those two have different sizes.
    /// The message received is assigned to the reference of String received by
    /// the caller.
    pub async fn recv_str(&mut self, _line: &mut String) -> Result<(), ReadHandlerError> {
        println!("\n\n\n\nHELLO\n\n\n");
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
