use indexmap::IndexMap;
use serde::Deserialize;

mod block;
mod costume;
mod list;
mod value;
mod variable;
pub mod op_codes;
pub use block::Block;
pub use costume::Costume;
pub use list::List;
pub use value::Value;
pub use variable::Variable;
pub use op_codes::OpCode;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    pub variables: IndexMap<String, Variable>,
    pub lists: IndexMap<String, List>,
    pub blocks: IndexMap<String, Block>,
    pub current_costume: usize,
    pub costumes: Vec<Costume>,
    pub volume: i32,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub size: Option<f32>,
    pub direction: Option<f32>,
}
