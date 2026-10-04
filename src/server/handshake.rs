use secrecy::SecretString;
use tokio::{
    io::{BufReader, BufWriter},
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

use crate::{
    shared::handshake::KeyExchangeError,
    socket_handling::{RecvHandler, WriteHandler},
};

pub async fn key_exchange(
    _stream: TcpStream,
    _shared_secret: SecretString,
) -> Result<
    // TODO: decide on the return type
    (
        WriteHandler<BufWriter<OwnedWriteHalf>>,
        RecvHandler<BufReader<OwnedReadHalf>>,
    ),
    KeyExchangeError,
> {
    // TODO:

    Err(KeyExchangeError::Fatal(anyhow::anyhow!("TODO")))
}
