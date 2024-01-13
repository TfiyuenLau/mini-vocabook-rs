mod test;

use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::Query;
use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{Extension, Json};
use serde_json::Value;
use common::dto::user_dto::{UserLoginDto, UserRegisterDto};
use common::entity::user;
use common::entity::user::Model;
use common::res::ResJson;
use service::user_service::{get_user_by_id, insert_user, login, update_user, update_user_password};
use crate::AppState;

// 用户登录处理器
pub async fn login_handler(
    Extension(state): Extension<Arc<AppState>>,
    user_login_dto: Result<Query<UserLoginDto>, QueryRejection>,
) -> ResJson<user::Model> {
    match user_login_dto {
        Ok(user_login_dto) => {
            let conn = &state.db_conn;
            match login(conn.to_owned(), user_login_dto.0).await {
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

// 用户个人信息处理器
pub async fn get_user_by_id_handler(
    Extension(state): Extension<Arc<AppState>>,
    user_id: Result<Query<HashMap<String, u64>>, QueryRejection>,
) -> ResJson<user::Model> {
    match user_id {
        Ok(user_id) => {
            let conn = &state.db_conn;
            match get_user_by_id(conn.to_owned(), *user_id.0.get("user_id").unwrap()).await {
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
pub async fn register_handler(
    Extension(state): Extension<Arc<AppState>>,
    user_json: Result<Json<Value>, JsonRejection>,
) -> ResJson<user::Model> {
    match user_json {
        Ok(user_json) => {
            let conn = &state.db_conn;
            let res = insert_user(conn.to_owned(), user_json.0).await;

            ResJson::success(res)
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 用户信息更新处理器
pub async fn update_user_password_handler(
    Extension(state): Extension<Arc<AppState>>,
    query: Result<Query<HashMap<String, String>>, QueryRejection>,
) -> ResJson<Model> {
    match query {
        Ok(query) => {
            let conn = &state.db_conn;
            let map = query.0;
            match update_user_password(conn.to_owned(), map.get("email").unwrap().to_string(), map.get("password").unwrap().to_string(), map.get("modify_pw").unwrap().to_string()).await {
                Some(res) => {
                    ResJson::success(res)
                }
                None => {
                    ResJson::error("密码错误".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}

// 用户信息更新处理器
pub async fn update_user_handler(
    Extension(state): Extension<Arc<AppState>>,
    user_json: Result<Json<Value>, JsonRejection>,
) -> ResJson<Model> {
    match user_json {
        Ok(user_json) => {
            let conn = &state.db_conn;
            let dto: UserRegisterDto = serde_json::from_value(user_json.0).unwrap();
            match update_user(conn.to_owned(), dto).await {
                Some(res) => {
                    ResJson::success(res)
                }
                None => {
                    ResJson::error("账号或密码错误".to_string())
                }
            }
        }
        Err(err) => {
            eprintln!("json error {:?}", err);
            ResJson::error(err.to_string())
        }
    }
}
