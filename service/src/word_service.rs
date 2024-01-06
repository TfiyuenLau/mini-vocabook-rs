use sea_orm::{DatabaseConnection, DbErr, EntityTrait, LoaderTrait, QueryOrder, QuerySelect};
use common::entity::prelude::{Word, Wordbook, WordWordbookMapping};
use common::entity::word;
use common::entity::word::Model;

// 获取前500个单词并返回列表
pub async fn get_all_word(db: DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let res: Result<Vec<Model>, DbErr> = Word::find()
        .order_by_asc(word::Column::WordId)
        .limit(500)
        .all(&db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    res
}

// 根据wordbook_id外键获取对应的word集合
pub async fn get_word_by_wordbook_id(db: DatabaseConnection, wordbook_id: u64) -> Result<Vec<Vec<Model>>, DbErr> {
    let wordbook = Wordbook::find_by_id(wordbook_id).all(&db).await;
    let res = wordbook.unwrap()
        .load_many_to_many(Word, WordWordbookMapping, &db)
        .await
        .map_err(|e| {
            eprintln!("Error fetching words: {:?}", e);
            DbErr::RecordNotFound(e.to_string())
        });

    res
}
