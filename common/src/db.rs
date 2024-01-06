use std::env;
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

// 获取SeaORM DB连接池
pub async fn get_db_conn() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not in .env file.");

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(128)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("mini_vocabook");

    let db_conn = Database::connect(opt).await.expect("database connect error");
    db_conn
}
