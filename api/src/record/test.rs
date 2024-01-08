#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sea_orm::DbErr;
    use common::db::get_db_conn;
    use common::dto::learning_record_dto::UploadRecordDto;
    use service::record_service::{have_user_word_id_record, insert_learning_record, increase_record_mastery_level};

    #[tokio::test]
    async fn test_have_user_word_id_record() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let dto = UploadRecordDto {
            user_id: 1,
            word_id: 12000,
            flag: false,
        };
        let model = have_user_word_id_record(conn, dto).await;
        println!("{:?}", model);

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_learning_record() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let dto = UploadRecordDto {
            user_id: 1,
            word_id: 2,
            flag: true,
        };
        let model = insert_learning_record(conn, dto).await;
        println!("{:?}", model);

        Ok(())
    }

    #[tokio::test]
    async fn test_increase_record_mastery_level() -> Result<(), DbErr> {
        dotenv().ok();

        let conn = get_db_conn().await;
        let dto = UploadRecordDto {
            user_id: 1,
            word_id: 2,
            flag: true,
        };
        let model = increase_record_mastery_level(conn, dto).await;
        println!("{:?}", model);

        Ok(())
    }
}
