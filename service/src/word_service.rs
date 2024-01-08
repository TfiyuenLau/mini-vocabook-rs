use sea_orm::{DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, QueryOrder, QuerySelect, Statement};
use common::entity::prelude::{Word};
use common::entity::{word};
use common::entity::word::Model;

// 单词查询测试
pub async fn get_all_words(db: DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let res: Result<Vec<Model>, DbErr> = Word::find()
        .order_by_asc(word::Column::WordId)
        .limit(50)
        .all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    res
}

// 通过word_id获取word对象
pub async fn get_word_by_id(db: DatabaseConnection, word_id: u64) -> Option<Model> {
    let res = Word::find_by_id(word_id)
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    res.unwrap()
}

// 查询待学习的单词集合
pub async fn get_learning_words(db: DatabaseConnection, user_id: u64, wordbook_id: u64, limit: u64) -> Result<Vec<Model>, DbErr> {
    // Row Sql Query
    let learning_word_list = Word::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            r#"
            SELECT w.*
            FROM word w
            JOIN word_wordbook_mapping wm ON w.word_id = wm.word_id
            WHERE wm.wordbook_id = ?
            AND NOT EXISTS (
                SELECT 1
                FROM learning_record lr
                WHERE lr.user_id = ?
                AND lr.word_id = w.word_id
            )
            ORDER BY w.word_id ASC
            LIMIT ?;
            "#,
            [wordbook_id.into(), user_id.into(), limit.into()],
        )).all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    learning_word_list
}

// 查询待学习的单词集合
pub async fn get_review_words(db: DatabaseConnection, user_id: u64, wordbook_id: u64, limit: u64) -> Result<Vec<Model>, DbErr> {
    let result = Word::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            r#"
            SELECT w.*
            FROM word w
            JOIN word_wordbook_mapping wm ON w.word_id = wm.word_id
            JOIN learning_record lr ON w.word_id = lr.word_id
            WHERE lr.user_id = ? AND wm.wordbook_id = ? AND lr.mastery_level < 3
            ORDER BY w.word_id ASC
            LIMIT ?;
            "#,
            [user_id.into(), wordbook_id.into(), limit.into()],
        )).all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    result
}
