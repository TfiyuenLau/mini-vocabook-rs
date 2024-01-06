mod test;

use std::collections::HashMap;
use std::sync::Arc;
use axum::Extension;
use axum::extract::{Query};
use axum::extract::rejection::QueryRejection;
use common::res::ResJson;
use common::entity::word::Model;
use service::word_service::{get_all_word, get_word_by_wordbook_id};
use crate::AppState;

pub async fn all_word_handler(Extension(state): Extension<Arc<AppState>>) -> ResJson<Vec<Model>> {
    let conn = &state.db_conn;
    let word_list = get_all_word(conn.to_owned()).await.unwrap();

    ResJson::success(word_list)
}

pub async fn words_by_wordbook_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    wordbook_id: Result<Query<HashMap<String, u64>>, QueryRejection>
) -> ResJson<Vec<Model>> {
    match wordbook_id {
        Ok(wordbook_id) => {
            let conn = &state.db_conn;
            match get_word_by_wordbook_id(conn.to_owned(), *(wordbook_id.0.get("wordbook_id").unwrap())).await {
                Ok(word_list) => {
                    ResJson::success(word_list.get(0).unwrap().to_owned())
                }
                Err(_) => {
                    ResJson::error("没有单词本对应的单词".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}
