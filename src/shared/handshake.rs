// TODO: maybe we want different variants
#[derive(thiserror::Error, Debug)]
pub enum KeyExchangeError {
    #[error(transparent)]
    Fatal(anyhow::Error),
    #[error(transparent)]
    NonFatal(anyhow::Error),
}

