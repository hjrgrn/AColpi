use acolpi::client::handshake::key_exchange;
use secrecy::SecretString;
use tokio::net::TcpStream;

use crate::utils::{SHARED_SECRET, spawn_app};

mod utils;

#[tokio::test]
async fn key_exchange_succeeds() {
    let app = spawn_app().await;
    let msg = String::from("Messaggio");

    let stream = TcpStream::connect(app.full_address()).await.unwrap();
    let (mut write_handler, _read_handler) = key_exchange(stream, SecretString::from(SHARED_SECRET))
        .await
        .unwrap();

    write_handler.write_str(&msg).await.unwrap();
    let msg_from_server = app.receive_from_server().await.unwrap();
    assert_eq!(msg, msg_from_server)
}
