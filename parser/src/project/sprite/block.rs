use super::op_codes::OpCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub opcode: OpCode,
    pub next: Option<String>,
    pub parent: Option<String>,
    pub shadow: bool,
    pub top_level: bool,
    pub x: Option<i32>,
    pub y: Option<i32>,
}
