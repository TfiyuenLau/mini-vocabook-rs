use sea_orm::IntoActiveValue;
use serde::{Deserialize, Serialize};
use crate::entity::user;

// 用户登录请求DTO
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}

// 用户注册请求DTO
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRegisterDto {
    pub email: String,
    pub username: String,
    pub password: String,
    pub wordbook_id: u64,
}

impl Into<user::ActiveModel> for UserRegisterDto {
    fn into(self) -> user::ActiveModel {
        user::ActiveModel {
            user_id: Default::default(),
            email: self.email.into_active_value(),
            username: self.username.into_active_value(),
            pw_hash: self.password.into_active_value(),
            wordbook_id: self.wordbook_id.into_active_value(),
            create_time: Default::default(),
            is_effective: 1.into_active_value(),
        }
    }
}
