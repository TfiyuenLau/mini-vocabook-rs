use paginate::Pages;
use sea_orm::{DatabaseBackend, DatabaseConnection, DbErr, EntityTrait, FromQueryResult, JsonValue, QueryOrder, Statement};
use serde_json::Number;
use common::entity::prelude::{Word, Wordbook};
use common::entity::{word, wordbook};
use common::entity::word::Model;

pub async fn get_wordbook_by_id(db: DatabaseConnection, wordbook_id: u64) -> Option<wordbook::Model> {
    let res = Wordbook::find_by_id(wordbook_id)
        .one(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    res.unwrap()
}

// 根据wordbook_id外键获取对应的word分页列表
pub async fn get_words_by_wordbook_id(db: DatabaseConnection, wordbook_id: u64, offset: usize) -> Result<Vec<Model>, DbErr> {
    // 查询wordbook_id对应的word列表
    let word_list = Wordbook::find_by_id(wordbook_id)
        .find_with_related(Word)
        .order_by_asc(word::Column::WordId)
        .all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    // 分页组件进行分页
    let res: Vec<Model> = word_list.unwrap().get(0).unwrap().to_owned().1;
    let pages = Pages::new(res.len(), 30);
    let page = pages.with_offset(offset);

    Ok(res[page.start..=page.end].to_owned())
}

// 获取单词本的单词数量
pub async fn get_wordbook_word_count(db: DatabaseConnection, wordbook_id: u64) -> Option<Number> {
    let wordbook_count = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            r#"
            SELECT
                COUNT(w.word_id) AS word_count
            FROM
                word_wordbook_mapping wm
            JOIN
                word w ON wm.word_id = w.word_id
            WHERE
                wm.wordbook_id = ?;
            "#,
            [wordbook_id.into()],
        ))
        .one(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    let value = wordbook_count.unwrap().unwrap();
    let option = value.get("word_count").unwrap().as_number();
    option.cloned()
}

// 获取用户的单词本学习进度
pub async fn get_wordbook_progress(db: DatabaseConnection, user_id: u64, wordbook_id: u64) -> Option<Number> {
    let progress = JsonValue::find_by_statement(Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            r#"
            SELECT
                COUNT(lr.word_id) AS proficient_word_count
            FROM
                learning_record lr
            JOIN
                user u ON lr.user_id = u.user_id
            WHERE
                u.user_id = ?
                AND u.wordbook_id = ?
                AND lr.mastery_level >= 1;
            "#,
            [user_id.into(), wordbook_id.into()],
        ))
        .one(&db).await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    let value = progress.unwrap().unwrap();
    let option = value.get("proficient_word_count").unwrap().as_number();
    option.cloned()
}
