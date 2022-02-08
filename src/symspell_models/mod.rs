use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarWord {
    pub str_similar_word: String,
    pub distance: i64,
    pub count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QryString {
    pub str_orig: String
}