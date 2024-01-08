use std::env;
use std::sync::Arc;
use axum::{Extension};
use sea_orm::{DatabaseConnection};
use common::db::get_db_conn;
use crate::routes::init;

mod routes;
mod user;
mod word;
mod wordbook;
mod record;

#[tokio::main]
pub async fn start() {
    // reading .env file
    let addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not in .env file.");

    // app state configuration
    let state = Arc::new(AppState {
        db_conn: get_db_conn().await
    });

    // start axum serve:http://localhost:3030/api/
    let app = init().layer(Extension(state));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_conn: DatabaseConnection,
}

