mod test;

use std::collections::HashMap;
use std::sync::Arc;
use axum::Extension;
use axum::extract::Query;
use axum::extract::rejection::QueryRejection;
use common::dto::word_dto::MemoryTestsWord;
use common::entity::word;
use common::res::ResJson;
use common::entity::word::Model;
use service::word_service::{get_all_words, get_learning_words, get_memory_tests_words, get_review_words, get_spelling_tests_words, get_word_by_id};
use crate::AppState;

pub async fn all_word_handler(Extension(state): Extension<Arc<AppState>>) -> ResJson<Vec<Model>> {
    let conn = &state.db_conn;
    let word_list = get_all_words(conn.to_owned()).await.unwrap();

    ResJson::success(word_list)
}

// 单词信息处理器
pub async fn word_by_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    word_id: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Model> {
    match word_id {
        Ok(word_id) => {
            let conn = &state.db_conn;
            match get_word_by_id(conn.to_owned(), *word_id.0.get("word_id").unwrap()).await {
                Some(word) => {
                    ResJson::success(word)
                }
                None => {
                    ResJson::error("未能查找到对应单词".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 待学习单词处理器
pub async fn learning_word_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Model>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let query_map: HashMap<String, u64> = query.0;
            match get_learning_words(
                conn.to_owned(),
                *query_map.get("user_id").unwrap(),
                *query_map.get("wordbook_id").unwrap(),
                *query_map.get("limit").unwrap()
            ).await {
                Ok(word) => {
                    ResJson::success(word)
                }
                Err(_) => {
                    ResJson::success(Vec::new()) // 返回一个空向量
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 待复习单词处理器
pub async fn review_word_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Model>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let query_map: HashMap<String, u64> = query.0;
            match get_review_words(
                conn.to_owned(),
                *query_map.get("user_id").unwrap(),
                *query_map.get("wordbook_id").unwrap(),
                *query_map.get("limit").unwrap()
            ).await {
                Ok(word) => {
                    ResJson::success(word)
                }
                Err(_) => {
                    ResJson::success(Vec::new())
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 单词记忆测验处理器
pub async fn memory_tests_words_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<MemoryTestsWord>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let query_map: HashMap<String, u64> = query.0;
            match get_memory_tests_words(
                conn.to_owned(),
                *query_map.get("user_id").unwrap(),
                *query_map.get("limit").unwrap()
            ).await {
                Ok(word) => {
                    ResJson::success(word)
                }
                Err(_) => {
                    ResJson::success(Vec::new())
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 单词拼写测验处理器
pub async fn spelling_tests_words_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<word::Model>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let query_map: HashMap<String, u64> = query.0;
            match get_spelling_tests_words(
                conn.to_owned(),
                *query_map.get("user_id").unwrap(),
                *query_map.get("limit").unwrap()
            ).await {
                Ok(word) => {
                    ResJson::success(word)
                }
                Err(_) => {
                    ResJson::success(Vec::new())
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}
