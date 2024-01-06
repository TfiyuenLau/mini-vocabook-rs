mod test;

use std::collections::HashMap;
use axum::extract::Query;
use axum::extract::rejection::QueryRejection;
use common::db::get_db_conn;
use common::res::ResJson;
use common::entity::word::Model;
use service::word_service::{get_all_word, get_word_by_wordbook_id};

pub async fn all_word_handler() -> ResJson<Vec<Model>> {
    let conn = get_db_conn().await;
    let word_list = get_all_word(conn).await.unwrap();

    ResJson::success(word_list)
}

pub async fn words_by_wordbook_id_handler(wordbook_id: Result<Query<HashMap<String, u64>>, QueryRejection>) -> ResJson<Vec<Model>> {
    match wordbook_id {
        Ok(wordbook_id) => {
            let conn = get_db_conn().await;
            match get_word_by_wordbook_id(conn, *(wordbook_id.0.get("wordbook_id").unwrap())).await {
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
