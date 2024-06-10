use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Header {
    pub name: String,
    pub value: String,
}
