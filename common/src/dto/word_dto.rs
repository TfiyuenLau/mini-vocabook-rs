use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryTestsWord {
    pub word_id: u64,
    pub word: String,
    pub phonogram: Option<String>,
    pub definition: Option<String>,
    pub option: Vec<String>,
}
