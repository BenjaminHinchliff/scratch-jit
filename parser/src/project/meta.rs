use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub semver: String,
    pub vm: String,
    pub agent: String,
}
