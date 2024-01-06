use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde_json::{json, Value};
use common::dto::user_dto::{UserLoginDto, UserRegisterDto};
use common::encrypt_password;
use common::entity::prelude::User;
use common::entity::user;
use common::entity::user::{ActiveModel, Model};

// 用户登录请求
pub async fn login(db: DatabaseConnection, mut user_login_dto: UserLoginDto) -> Option<Model> {
    user_login_dto.password = encrypt_password(user_login_dto.password);

    let result = User::find()
        .filter(user::Column::Email.eq(user_login_dto.email))
        .filter(user::Column::PwHash.eq(user_login_dto.password))
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        })
        .unwrap();

    result
}

// 通过ID获取user对象
pub async fn get_user_by_id(db: DatabaseConnection, user_id: u64) -> Option<Model> {
    let result = User::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        })
        .unwrap();

    result
}

// 新增一个用户
pub async fn insert_user(db: DatabaseConnection, mut user_json: Value) -> user::Model {
    // 加密user_json中的密码
    if let Some(obj) = user_json.as_object_mut() {
        if let Some(pw_hash) = obj.get_mut("pw_hash") {
            *pw_hash = json!(encrypt_password((*pw_hash).to_string()));
        }
    }

    // 通过Json DTO构建ActiveModel对象
    let user_dto: UserRegisterDto = serde_json::from_value(user_json).unwrap();
    let model: ActiveModel = user_dto.into();

    // 发送新增请求
    let result = model.insert(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotInserted
        });

    result.unwrap()
}
