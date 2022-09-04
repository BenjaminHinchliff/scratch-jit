pub mod project;
pub use project::Project;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn deserialize_works_formatted() {
        let json = fs::read_to_string("tests/formatted.json").unwrap();
        let project: Project = serde_json::from_str(&json).unwrap();

        insta::assert_debug_snapshot!(&project);
    }
}
