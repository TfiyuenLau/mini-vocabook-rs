#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sea_orm::DbErr;
    use common::db::get_db_conn;
    use service::word_service::{get_learning_words, get_word_by_id, get_review_words, get_memory_tests_words};

    #[tokio::test]
    async fn test_get_word_by_id() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_word_by_id(conn, 1).await.unwrap();
        println!("{:?}", word_list);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_learning_words() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_learning_words(conn, 1, 3, 30).await;
        println!("{:?}", word_list.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_review_words() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_review_words(conn, 1, 3, 60).await;
        println!("{:?}", word_list.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_get_memory_tests_words() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let word_list = get_memory_tests_words(conn, 1, 30).await;
        println!("{:?}", word_list.unwrap());

        Ok(())
    }
}
