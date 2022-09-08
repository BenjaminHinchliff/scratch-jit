use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
}
