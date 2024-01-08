use sea_orm::IntoActiveValue;
use serde::{Deserialize, Serialize};
use crate::entity::learning_record::ActiveModel;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadRecordDto {
    pub user_id: u64,
    pub word_id: u64,
    pub flag: bool, // 标识熟练程度上升或下降
}

impl Into<ActiveModel> for UploadRecordDto {
    fn into(self) -> ActiveModel {
        ActiveModel {
            record_id: Default::default(),
            user_id: self.user_id.into_active_value(),
            word_id: self.word_id.into_active_value(),
            mastery_level: Default::default(),
            learning_dt: Default::default(),
            is_effective: Default::default(),
        }
    }
}
