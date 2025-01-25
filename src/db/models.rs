use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo_model {
    pub title: String,
    pub description: String,
    pub completed: bool,
}