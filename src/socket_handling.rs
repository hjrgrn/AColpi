use tokio::io::{self, AsyncRead, AsyncWrite};

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

#[derive(thiserror::Error, Debug)]
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
        Err(ReadHandlerError::WIP)
    }
}
