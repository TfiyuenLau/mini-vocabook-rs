use std::collections::HashMap;
use axum::extract::Query;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::Json;
use serde_json::Value;
use common::db::get_db_conn;
use common::dto::user_dto::{UserLoginDto};
use common::entity::user;
use common::res::ResJson;
use service::user_service::{get_user_by_id, insert_user, login};

// 用户登录处理器
pub async fn login_handler(user_login_dto: Result<Query<UserLoginDto>, QueryRejection>) -> ResJson<user::Model> {
    match user_login_dto {
        Ok(user_login_dto) => {
            let conn = get_db_conn().await;
            match login(conn, user_login_dto.0).await {
                Some(user) => {
                    ResJson::success(user)
                }
                None => {
                    ResJson::error("账号或密码错误".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("query error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 获取用户信息处理器
pub async fn get_user_by_id_handler(user_id: Result<Query<HashMap<String, u64>>, QueryRejection>) -> ResJson<user::Model> {
    match user_id {
        Ok(user_id) => {
            let conn = get_db_conn().await;
            match get_user_by_id(conn, *user_id.0.get("user_id").unwrap()).await {
                Some(user) => {
                    ResJson::success(user)
                }
                None => {
                    ResJson::error("用户ID不存在".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("handler error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 用户注册处理器
pub async fn register_handler(user_json: Result<Json<Value>, JsonRejection>) -> ResJson<user::Model> {
    match user_json {
        Ok(user_json) => {
            let conn = get_db_conn().await;
            let res = insert_user(conn, user_json.0).await;

            ResJson::success(res)
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}