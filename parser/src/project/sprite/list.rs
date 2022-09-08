use serde::Deserialize;

use super::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    pub name: String,
    pub data: Vec<Value>,
}
