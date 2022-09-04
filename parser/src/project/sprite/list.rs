use serde::Deserialize;

use super::Value;

#[derive(Debug, Deserialize)]
pub struct List {
    pub name: String,
    pub data: Vec<Value>,
}
