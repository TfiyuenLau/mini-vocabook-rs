mod test;

use std::collections::HashMap;
use std::sync::Arc;
use axum::Extension;
use axum::extract::Query;
use axum::extract::rejection::QueryRejection;
use common::entity::word::Model;
use common::entity::wordbook;
use common::res::ResJson;
use service::wordbook_service::{get_wordbook_by_id, get_wordbook_progress, get_wordbook_word_count, get_words_by_wordbook_id};
use crate::AppState;

pub async fn wordbook_by_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    wordbook_id: Result<Query<HashMap<String, u64>>, QueryRejection>
) -> ResJson<wordbook::Model> {
    match wordbook_id {
        Ok(wordbook_id) => {
            let conn = &state.db_conn;
            match get_wordbook_by_id(conn.to_owned(), *(wordbook_id.0.get("wordbook_id").unwrap())).await {
                Some(wordbook) => {
                    ResJson::success(wordbook)
                }
                None => {
                    ResJson::error("没有找到对应的单词本".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

pub async fn words_by_wordbook_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    params: Result<Query<HashMap<String, u64>>, QueryRejection>
) -> ResJson<Vec<Model>> {
    match params {
        Ok(params) => {
            let conn = &state.db_conn;
            match get_words_by_wordbook_id(conn.to_owned(), *(params.0.get("wordbook_id").unwrap()), *(params.0.get("offset").unwrap()) as usize).await {
                Ok(word_list) => {
                    ResJson::success(word_list)
                }
                Err(_) => {
                    ResJson::error("没有找到单词本对应的单词".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

pub async fn wordbook_word_count_handler(
    Extension(state): Extension<Arc<AppState>>,
    params: Result<Query<HashMap<String, u64>>, QueryRejection>
) -> ResJson<u64> {
    match params {
        Ok(params) => {
            let conn = &state.db_conn;
            match get_wordbook_word_count(conn.to_owned(), *(params.0.get("wordbook_id").unwrap())).await {
                Some(count) => {
                    ResJson::success(count.as_u64().unwrap())
                }
                None => {
                    ResJson::error("没有找到单词本对应的单词".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

pub async fn wordbook_progress_handler(
    Extension(state): Extension<Arc<AppState>>,
    params: Result<Query<HashMap<String, u64>>, QueryRejection>
) -> ResJson<u64> {
    match params {
        Ok(params) => {
            let conn = &state.db_conn;
            let map = params.0;
            match get_wordbook_progress(conn.to_owned(), *(map.get("user_id").unwrap()), *(map.get("wordbook_id").unwrap())).await {
                Some(count) => {
                    ResJson::success(count.as_u64().unwrap())
                }
                None => {
                    ResJson::error("没有找到单词本对应的单词".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}
