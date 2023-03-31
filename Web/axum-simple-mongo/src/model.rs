use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: Uuid,
    pub name: String,
    pub age: i32,
}
