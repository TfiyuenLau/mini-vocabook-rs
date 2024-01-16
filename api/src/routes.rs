use axum::Router;
use axum::routing::{get, post};
use crate::record::{date_check_in_statistic_handler, get_14days_record_statistic_handler, insert_or_update_record_handler, mastery_level_statistic_handler, word_cloud_statistic_handler};
use crate::user::{get_user_by_id_handler, login_handler, register_handler, update_user_handler, update_user_password_handler};
use crate::word::{all_word_handler, learning_word_handler, memory_tests_words_handler, review_word_handler, word_by_id_handler};
use crate::wordbook::{wordbook_by_id_handler, wordbook_progress_handler, wordbook_word_count_handler, words_by_wordbook_id_handler};

/// 初始化API路由
pub fn init() -> Router {
    let user_routes = Router::new()
        .nest("/user", get_user_routes());

    let word_routes = Router::new()
        .nest("/word", get_word_routes());

    let wordbook_routes = Router::new()
        .nest("/wordbook", get_wordbook_routes());

    let learning_record_routes = Router::new()
        .nest("/record", get_learning_record_routes());

    let app_routes = Router::new()
        .nest("/api", user_routes)
        .nest("/api", word_routes)
        .nest("/api", wordbook_routes)
        .nest("/api", learning_record_routes);

    return app_routes;
}

/// 用户二级路由
pub fn get_user_routes() -> Router {
    Router::new()
        .route("/login", get(login_handler))
        .route("/get_user_by_id", get(get_user_by_id_handler))
        .route("/register", post(register_handler))
        .route("/update_user_password", post(update_user_password_handler))
        .route("/update_user", post(update_user_handler))
}

/// 单词二级路由
pub fn get_word_routes() -> Router {
    Router::new()
        .route("/get_all_word", get(all_word_handler))
        .route("/get_word_by_id", get(word_by_id_handler))
        .route("/get_learning_words", get(learning_word_handler))
        .route("/get_review_words", get(review_word_handler))
        .route("/get_memory_tests_words", get(memory_tests_words_handler))
}

/// 单词本二级路由
pub fn get_wordbook_routes() -> Router {
    Router::new()
        .route("/get_wordbook_by_id", get(wordbook_by_id_handler))
        .route("/get_words_by_wordbook_id", get(words_by_wordbook_id_handler))
        .route("/get_wordbook_word_count", get(wordbook_word_count_handler))
        .route("/get_wordbook_progress", get(wordbook_progress_handler))
}

/// 学习记录二级路由
pub fn get_learning_record_routes() -> Router {
    Router::new()
        .route("/get_14days_record_statistic", get(get_14days_record_statistic_handler))
        .route("/get_mastery_level_statistic", get(mastery_level_statistic_handler))
        .route("/get_word_cloud_statistic", get(word_cloud_statistic_handler))
        .route("/get_date_check_in_statistic", get(date_check_in_statistic_handler))
        .route("/insert_or_update_record", post(insert_or_update_record_handler))
}
