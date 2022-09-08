use serde::Deserialize;

pub mod sprite;
pub use sprite::Target;
mod meta;
pub use meta::Meta;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub targets: Vec<Target>,
    pub extensions: Vec<String>,
    pub meta: Meta,
}
