#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sea_orm::DbErr;
    use common::db::get_db_conn;
    use service::user_service::get_user_by_id;

    #[tokio::test]
    async fn test_get_user_by_id() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let user = get_user_by_id(conn, 1).await.unwrap();
        println!("{:?}", user);

        Ok(())
    }
}
