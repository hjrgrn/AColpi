use hmac::{Hmac, KeyInit};
use rand::{Rng, distributions::Alphanumeric, rngs::OsRng};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs1::EncodeRsaPublicKey, pkcs8::LineEnding};
use secrecy::{ExposeSecret, SecretString};
use sha2::Sha256;
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
    stream: TcpStream,
    shared_secret: SecretString,
) -> Result<
    // TODO: decide on the return type
    (
        WriteHandler<BufWriter<OwnedWriteHalf>>,
        ReadHandler<BufReader<OwnedReadHalf>>,
    ),
    KeyExchangeError,
> {
    // TODO:

    let (read, write) = stream.into_split();
    let reader = BufReader::new(read);
    let writer = BufWriter::new(write);
    let mut write_handler = WriteHandler::new(writer);
    let mut read_handler = ReadHandler::new(reader);

    let mut hmac = Hmac::<Sha256>::new_from_slice(shared_secret.expose_secret().as_bytes())
        .map_err(|e| KeyExchangeError::NonFatal(e.into()))?;

    let mut rng = OsRng::default();
    // generate nonce
    let mut body: String = rng
        .sample_iter(Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();
    let sent_nonce = body.clone();

    let bits = 1024;
    let priv_key =
        RsaPrivateKey::new(&mut rng, bits).map_err(|e| KeyExchangeError::NonFatal(e.into()))?;
    let pub_key = RsaPublicKey::from(&priv_key);
    let pk_str = pub_key
        .to_pkcs1_pem(LineEnding::default())
        .map_err(|e| KeyExchangeError::Fatal(e.into()))?;
    // nonce + pem, size 24 + 251
    body.push_str(&pk_str);

    // TODOFIRST: build these
    write_handler
        .write_str(&body)
        .await
        .map_err(|e| KeyExchangeError::NonFatal(e.into()))?;

    let res = read_handler
        .recv_bytes()
        .await
        .map_err(|e| KeyExchangeError::NonFatal(e.into()))?;
    if res.len() != 184 {
        return Err(KeyExchangeError::NonFatal(anyhow::anyhow!(
            "Malformed packet received during key_exchange."
        )));
    }

    Err(KeyExchangeError::Fatal(anyhow::anyhow!("TODO")))
}
