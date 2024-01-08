#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sea_orm::DbErr;
    use common::db::get_db_conn;
    use service::wordbook_service::{get_wordbook_progress, get_wordbook_word_count, get_words_by_wordbook_id};

    #[tokio::test]
    async fn test_get_words_by_wordbook_id() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_words_by_wordbook_id(conn, 1, 1).await;
        println!("{:?}", word_list.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_wordbook_progress() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let wordbook_count = get_wordbook_word_count(conn.clone(), 1).await;
        let progress = get_wordbook_progress(conn.clone(), 1, 3).await;
        println!("wordbook_count:{:?}, progress:{:?}", wordbook_count.unwrap().as_u64().unwrap(), progress.unwrap().as_u64().unwrap());

        Ok(())
    }
}
