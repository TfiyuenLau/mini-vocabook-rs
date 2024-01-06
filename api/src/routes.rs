use axum::Router;
use axum::routing::{get, post};
use crate::user::{get_user_by_id_handler, login_handler, register_handler};
use crate::word::{all_word_handler, words_by_wordbook_id_handler};

/// 初始化API路由
pub fn init() -> Router {
    let user_routes = Router::new()
        .nest("/user", get_user_routes());

    let word_routes = Router::new()
        .nest("/word", get_word_routes());

    let app_routes = Router::new()
        .nest("/api", user_routes)
        .nest("/api", word_routes);

    return app_routes;
}

/// 用户二级路由
pub fn get_user_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/get_user_by_id", get(get_user_by_id_handler))
        .route("/register", post(register_handler))
}

/// 单词二级路由
pub fn get_word_routes() -> Router {
    Router::new()
        .route("/get_all_word", get(all_word_handler))
        .route("/get_words_by_wordbook_id", get(words_by_wordbook_id_handler))
}
