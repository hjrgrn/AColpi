// TODO: split the content into semantic sections

use amplify::Getters;

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

#[derive(Getters)]
pub struct Application {
    #[getter(as_copy)]
    port: u16,
    // TODO: server: Server // NOTE: we will have to drop the server, that's why we need it
}

impl Application {
    fn build() -> Self {
        // TODO:
        Application { port: 0 }
    }
    pub async fn run_until_stopped(self) {
        // TODO:
    }
}

pub async fn spawn_app() -> TestApp {
    let app = Application::build();
    let port = app.port();
    let _ = tokio::spawn(app.run_until_stopped());

    // TODO:
    TestApp {
        address: "127.0.0.1".into(),
        port,
    }
}
