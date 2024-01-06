#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sea_orm::DbErr;
    use common::db::get_db_conn;
    use service::word_service::get_word_by_wordbook_id;

    #[tokio::test]
    async fn test_get_word_by_wordbook_id() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_word_by_wordbook_id(conn, 1).await.unwrap();
        println!("{:?}", word_list.get(0).unwrap());

        Ok(())
    }
}