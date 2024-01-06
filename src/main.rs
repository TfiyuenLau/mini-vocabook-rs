use dotenvy::dotenv;
use tracing::Level;
use api;

/// Axum服务启动入口
fn main() {
    // SeaORM debug log
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_test_writer()
        .init();

    // start server
    dotenv().ok();
    api::start();
}
