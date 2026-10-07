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
    socket_handling::{ReadHandler, WriteHandler},
};

pub async fn key_exchange(
    _stream: TcpStream,
    _shared_secret: SecretString,
) -> Result<
    // TODO: decide on the return type
    (
        WriteHandler<BufWriter<OwnedWriteHalf>>,
        ReadHandler<BufReader<OwnedReadHalf>>,
    ),
    KeyExchangeError,
> {
    // TODO:

    Err(KeyExchangeError::Fatal(anyhow::anyhow!("TODO")))
}
