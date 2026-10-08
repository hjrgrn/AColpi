use acolpi::server::handshake::key_exchange;
use amplify::Getters;
use secrecy::SecretString;
use tokio::{
    io,
    net::TcpListener,
    sync::oneshot::{self, error::RecvError},
};

pub const SHARED_SECRET: &str = "shared_secret";

pub struct TestApp {
    address: String,
    port: u16,
    shared_secret: SecretString,
    receiver: oneshot::Receiver<String>,
}

impl TestApp {
    pub fn full_address(&self) -> String {
        format!("{}:{}", &self.address, self.port)
    }

    pub async fn receive_from_server(self) -> Result<String, RecvError> {
        self.receiver.await
    }
}

#[derive(Getters)]
pub struct Server {
    #[getter(as_copy)]
    port: u16,
    address: String,
    #[getter(skip)]
    listener: TcpListener,
    #[getter(as_clone)]
    shared_secret: SecretString,
    /// Transmits the package received by the server.
    #[getter(skip)]
    transmitter: oneshot::Sender<String>, // TODO: we may need more than a oneshot.
}

impl Server {
    pub async fn build() -> io::Result<(Self, oneshot::Receiver<String>)> {
        let address = String::from("127.0.0.1");
        let listener = TcpListener::bind(&format!("{address}:0")).await?;
        let port = listener.local_addr()?.port();
        let shared_secret = SecretString::from(SHARED_SECRET);
        let (transmitter, receiver) = oneshot::channel();
        Ok((
            Server {
                port,
                address,
                listener,
                shared_secret,
                transmitter,
            },
            receiver,
        ))
    }

    /// Accept a connection. Exchanges the keys with the client that required the
    /// connection. Waits for a packet to arrive. Transmits the content of the packet
    /// through the channel [`Server::transmitter`].
    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        let (stream, _) = self.listener.accept().await?;
        let (_write_handler, mut read_handler) = key_exchange(stream, self.shared_secret).await?;
        let mut line = String::new();
        read_handler.recv_str(&mut line).await?;
        self.transmitter
            .send(line)
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}

pub async fn spawn_app() -> TestApp {
    let (app, receiver) = Server::build().await.unwrap();
    let port = app.port();
    let shared_secret = app.shared_secret();
    let address = app.address().to_string();
    let _ = tokio::spawn(app.run_until_stopped());

    // TODO:
    TestApp {
        address,
        port,
        shared_secret,
        receiver,
    }
}
