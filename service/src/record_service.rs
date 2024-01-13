use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JsonValue, QueryFilter, Statement};
use sea_orm::ActiveValue::Set;
use serde_json::Value;
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

// 获取近14日学习记录的统计信息
pub async fn get_14days_record_statistic(db: DatabaseConnection, user_id: u64) -> Result<Vec<Value>, DbErr> {
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        r#"
        SELECT
            DATE(date_range.date) AS study_date,
            COALESCE(COUNT(learning_record.record_id), 0) AS study_count
        FROM
            (SELECT CURDATE() - INTERVAL n DAY AS date FROM (SELECT 0 AS n UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6 UNION ALL SELECT 7 UNION ALL SELECT 8 UNION ALL SELECT 9 UNION ALL SELECT 10 UNION ALL SELECT 11 UNION ALL SELECT 12 UNION ALL SELECT 13) AS nums) AS date_range
        LEFT JOIN learning_record ON (DATE(learning_record.create_time) = date_range.date OR DATE(learning_record.update_time) = date_range.date) AND learning_record.user_id = ?
        GROUP BY study_date
        ORDER BY study_date ASC;
        "#,
        [user_id.into()],
    ))
        .all(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    result
}

// 获取单词本各个熟练程度单词数量统计值
pub async fn get_mastery_level_statistic(db: DatabaseConnection, user_id: u64, wordbook_id: u64) -> Result<Vec<Value>, DbErr> {
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        r#"
        SELECT
            COALESCE(mastery_level, -1) AS mastery_level,
            COUNT(word.word_id) AS word_count
        FROM
            word
        LEFT JOIN learning_record ON word.word_id = learning_record.word_id AND learning_record.user_id = ?
        LEFT JOIN user ON learning_record.user_id = user.user_id
        WHERE
            (user.wordbook_id IS NULL OR user.wordbook_id = ?)
        GROUP BY
            mastery_level
        ORDER BY
            mastery_level;
        "#,
        [user_id.into(), wordbook_id.into()],
    ))
        .all(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    result
}

// 获取词云统计数据
pub async fn get_word_cloud_statistic(db: DatabaseConnection, user_id: u64) -> Result<Vec<Value>, DbErr> {
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        r#"
        SELECT
            word.word AS name
        FROM
            learning_record AS lr
        JOIN word ON lr.word_id = word.word_id
        WHERE
            lr.user_id = ?
        ORDER BY
            lr.create_time DESC
        LIMIT 5;
        "#,
        [user_id.into()],
    ))
        .all(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    result
}

// 获取日期分组的打卡统计数据
pub async fn get_date_check_in_statistic(db: DatabaseConnection, user_id: u64, limit: u64) -> Result<Vec<Value>, DbErr> {
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        r#"
        SELECT
            study_date,
            COUNT(record_id) AS study_count
        FROM
            (
                SELECT
                    record_id,
                    DATE(create_time) AS study_date
                FROM
                    learning_record
                WHERE
                    user_id = ?
                UNION
                SELECT
                    record_id,
                    DATE(update_time) AS study_date
                FROM
                    learning_record
                WHERE
                    user_id = ?
            ) AS combined_records
        GROUP BY
            study_date
        ORDER BY
            study_date DESC
        LIMIT ?;
        "#,
        [user_id.into(), user_id.into(), limit.into()],
    ))
        .all(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    result
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
