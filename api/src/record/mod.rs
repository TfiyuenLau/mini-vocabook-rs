mod test;

use std::collections::HashMap;
use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::Query;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use serde_json::Value;
use common::dto::learning_record_dto::UploadRecordDto;
use common::entity::{learning_record};
use common::res::ResJson;
use service::record_service::{have_user_word_id_record, insert_learning_record, increase_record_mastery_level, decrease_record_mastery_level, get_14days_record_statistic, get_mastery_level_statistic, get_word_cloud_statistic, get_date_check_in_statistic};
use crate::AppState;

// 获取用户近14日的学习记录
pub async fn get_14days_record_statistic_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Value>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            match get_14days_record_statistic(conn.to_owned(), *(query.0.get("user_id").unwrap())).await {
                Ok(res) => {
                    ResJson::success(res)
                }
                Err(err) => {
                    ResJson::error(err.to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 获取用户对应单词本的熟练程度统计记录
pub async fn mastery_level_statistic_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Value>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let map = query.0;
            match get_mastery_level_statistic(conn.to_owned(), *(map.get("user_id").unwrap()), *(map.get("wordbook_id").unwrap())).await {
                Ok(res) => {
                    ResJson::success(res)
                }
                Err(err) => {
                    ResJson::error(err.to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 获取用户学习记录的词云图
pub async fn word_cloud_statistic_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Value>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            match get_word_cloud_statistic(conn.to_owned(), *(query.0.get("user_id").unwrap())).await {
                Ok(res) => {
                    ResJson::success(res)
                }
                Err(err) => {
                    ResJson::error(err.to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 获取用户的日期分组打卡条数
pub async fn date_check_in_statistic_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<Vec<Value>> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            match get_date_check_in_statistic(conn.to_owned(), *(query.0.get("user_id").unwrap()), *(query.0.get("limit").unwrap())).await {
                Ok(res) => {
                    ResJson::success(res)
                }
                Err(err) => {
                    ResJson::error(err.to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

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
