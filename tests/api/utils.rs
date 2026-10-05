use acolpi::server::handshake::key_exchange;
use amplify::Getters;
use secrecy::SecretString;
use tokio::{io, net::TcpListener};

const SHARED_SECRET: &str = "shared_secret";

#[derive(Getters)]
pub struct TestApp {
    address: String,
    port: u16,
    shared_secret: SecretString,
}

#[derive(Getters)]
pub struct Application {
    #[getter(as_copy)]
    port: u16,
    #[getter(skip)]
    listener: TcpListener,
    #[getter(as_clone)]
    shared_secret: SecretString,
}

impl Application {
    pub async fn build() -> io::Result<Self> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();
        let shared_secret = SecretString::from(SHARED_SECRET);
        Ok(Application {
            port,
            listener,
            shared_secret,
        })
    }

    pub async fn run_until_stopped(self) -> anyhow::Result<()> {
        let (stream, _) = self.listener.accept().await?;
        let (_write_handler, _read_handler) = key_exchange(stream, self.shared_secret).await?;
        // TODO: write something, receive something?
        Ok(())
    }
}

pub async fn spawn_app() -> TestApp {
    let app = Application::build().await.unwrap();
    let port = app.port();
    let shared_secret = app.shared_secret();
    let _ = tokio::spawn(app.run_until_stopped());

    // TODO:
    TestApp {
        address: "127.0.0.1".into(),
        port,
        shared_secret,
    }
}
