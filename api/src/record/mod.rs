mod test;

use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::rejection::JsonRejection;
use serde_json::Value;
use common::dto::learning_record_dto::UploadRecordDto;
use common::entity::{learning_record};
use common::res::ResJson;
use service::record_service::{have_user_word_id_record, insert_learning_record, increase_record_mastery_level, decrease_record_mastery_level};
use crate::AppState;

// 插入或更新学习记录处理器
pub async fn insert_or_update_record_handler(
    Extension(state): Extension<Arc<AppState>>,
    dto_json: Result<Json<Value>, JsonRejection>,
) -> ResJson<learning_record::Model> {
    match dto_json {
        Ok(dto_json) => {
            let conn = &state.db_conn;
            let dto: UploadRecordDto = serde_json::from_value(dto_json.0).unwrap();
            let res = match have_user_word_id_record(conn.to_owned(), dto.clone()).await {
                Some(_) => {
                    if dto.flag {
                        increase_record_mastery_level(conn.to_owned(), dto.clone()).await
                    } else {
                        decrease_record_mastery_level(conn.to_owned(), dto.clone()).await
                    }
                }
                None => {
                    insert_learning_record(conn.to_owned(), dto.clone()).await
                }
            }.unwrap();

            ResJson::success(res)
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}
