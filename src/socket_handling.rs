use tokio::io::{AsyncRead, AsyncWrite};

/// # `WriteHandler`
///
/// This struct handles the writing side of a socket.
/// It takes care also of encryption and message integrity.
/// It acts differently based on the presence of a `cipher`
/// in its fields.
// XXX: comment.
// IDEA: this could be a state machine, based on the fact that is using encription or not.
pub struct WriteHandler<T: AsyncWrite + Unpin + Send> {
    writer: T,
}

/// # `ReadHandler`
///
/// This struct handles the receiving side of a socket
/// It takes care also of encryption and message integrity.
/// It acts differently based on the presence of a `cipher`
/// in its fields.
// XXX: comment
pub struct ReadHandler<T: AsyncRead + Unpin + Send> {
    reader: T,
}
