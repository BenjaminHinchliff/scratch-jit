use serde::Deserialize;

use super::Value;

#[derive(Debug, Deserialize)]
pub struct Variable {
    pub name: String,
    pub value: Value,
}
