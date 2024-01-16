use sea_orm::{DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JsonValue, QueryOrder, QuerySelect, Statement};
use common::dto::word_dto::MemoryTestsWord;
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

// 获取用户对应的记忆测验单词列表
pub async fn get_memory_tests_words(db: DatabaseConnection, user_id: u64, limit: u64) -> Result<Vec<MemoryTestsWord>, DbErr> {
    // 查询待复习的单词以及随机错误释义
    let result = JsonValue::find_by_statement(Statement::from_sql_and_values(
        DatabaseBackend::MySql,
        r#"
        SELECT
            DISTINCT word.word_id AS word_id,
            word.word AS word,
            word.phonogram AS phonogram,
            word.definition AS definition,
            (SELECT w.definition FROM word w WHERE w.word_id != word.word_id ORDER BY RAND() LIMIT 1) AS option_1,
            (SELECT w.definition FROM word w WHERE w.word_id != word.word_id ORDER BY RAND() LIMIT 1) AS option_2,
            (SELECT w.definition FROM word w WHERE w.word_id != word.word_id ORDER BY RAND() LIMIT 1) AS option_3
        FROM
            learning_record AS lr
        JOIN
            word ON lr.word_id = word.word_id
        WHERE
            lr.user_id = ?
            AND lr.mastery_level < 3
        ORDER BY
            RAND()
        LIMIT ?;
        "#,
        [user_id.into(), limit.into()],
    ))
        .all(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    // 解析并封装数据列表
    let mut word_list: Vec<MemoryTestsWord> = Vec::new();
    for value in result.unwrap() {
        let word = MemoryTestsWord {
            word_id: value.get("word_id").unwrap().as_u64().unwrap(),
            word: value.get("word").unwrap().as_str().unwrap().to_string(),
            phonogram: Some(value.get("phonogram").unwrap().as_str().unwrap().to_string()),
            definition: Some(value.get("definition").unwrap().as_str().unwrap().to_string()),
            option: vec![
                value.get("option_1").unwrap().as_str().unwrap().to_string(),
                value.get("option_2").unwrap().as_str().unwrap().to_string(),
                value.get("option_3").unwrap().as_str().unwrap().to_string(),
            ],
        };
        word_list.push(word);
    }

    Ok(word_list)
}
