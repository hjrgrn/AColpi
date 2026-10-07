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
    (
        WriteHandler<BufWriter<OwnedWriteHalf>>,
        ReadHandler<BufReader<OwnedReadHalf>>,
    ),
    KeyExchangeError,
> {
    Err(KeyExchangeError::Fatal(anyhow::anyhow!("TODO:")))
}
