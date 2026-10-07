use crate::utils::spawn_app;

mod utils;

#[tokio::test]
async fn key_exchange_succeeds() {
    let app = spawn_app().await;
}
