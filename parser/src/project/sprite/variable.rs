use serde::Deserialize;

use super::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct Variable {
    pub name: String,
    pub value: Value,
}
