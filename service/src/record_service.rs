use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::Set;
use common::dto::learning_record_dto::UploadRecordDto;
use common::entity::learning_record;
use common::entity::learning_record::Model;
use common::entity::prelude::LearningRecord;

// 判断是否存在对应的单词学习记录
pub async fn have_user_word_id_record(db: DatabaseConnection, dto: UploadRecordDto) -> Option<Model> {
    let result = LearningRecord::find()
        .filter(learning_record::Column::UserId.eq(dto.user_id))
        .filter(learning_record::Column::WordId.eq(dto.word_id))
        .one(&db)
        .await;

    result.unwrap()
}

// 插入一条learning_record
pub async fn insert_learning_record(db: DatabaseConnection, dto: UploadRecordDto) -> Result<Model, DbErr> {
    let record: learning_record::ActiveModel = dto.into();

    record.insert(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotInserted
        })
}

// 升级一条learning_record的熟练等级
pub async fn increase_record_mastery_level(db: DatabaseConnection, dto: UploadRecordDto) -> Result<Model, DbErr> {
    let record = LearningRecord::find()
        .filter(learning_record::Column::UserId.eq(dto.user_id))
        .filter(learning_record::Column::WordId.eq(dto.word_id))
        .one(&db).await.unwrap().expect("learning record not found");

    if record.mastery_level < 3 {
        let mut record: learning_record::ActiveModel = record.into();
        record.mastery_level = Set(record.mastery_level.unwrap() + 1);

        record.update(&db).await
    } else {
        Ok(record)
    }
}

// 降低一条learning_record的熟练等级
pub async fn decrease_record_mastery_level(db: DatabaseConnection, dto: UploadRecordDto) -> Result<Model, DbErr> {
    let record = LearningRecord::find()
        .filter(learning_record::Column::UserId.eq(dto.user_id))
        .filter(learning_record::Column::WordId.eq(dto.word_id))
        .one(&db).await.unwrap().expect("learning record not found");

    if record.mastery_level > 0 {
        let mut record: learning_record::ActiveModel = record.into();
        record.mastery_level = Set(record.mastery_level.unwrap() - 1);

        record.update(&db).await
    } else {
        Ok(record)
    }
}
