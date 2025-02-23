//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.10

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "wordbook")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub wordbook_id: u64,
    pub book_name: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub introduction: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user::Entity")]
    User,
    #[sea_orm(has_many = "super::word_wordbook_mapping::Entity")]
    WordWordbookMapping,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::word_wordbook_mapping::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WordWordbookMapping.def()
    }
}

impl Related<super::word::Entity> for Entity {
    fn to() -> RelationDef {
        super::word_wordbook_mapping::Relation::Word.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::word_wordbook_mapping::Relation::Wordbook.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
