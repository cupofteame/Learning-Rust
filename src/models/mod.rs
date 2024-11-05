use serde::{Deserialize, Serialize};

// Message struct for handling echo request & response
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
} 