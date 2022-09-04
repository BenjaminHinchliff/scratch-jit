use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f32),
}
