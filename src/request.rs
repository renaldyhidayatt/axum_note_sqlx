use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteRequest {
    pub title: String,
    pub content: String,
}
