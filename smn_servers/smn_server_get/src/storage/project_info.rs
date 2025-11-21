use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectInfoConfig {
    pub projects: Vec<ProjectEntry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectEntry {
    pub name: String,
    pub key: String,
    pub description: String,
    pub version_latest: String,
    pub version_changelog: Vec<ProjectVersionLog>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectVersionLog {
    pub version: String,
    pub changes: Vec<String>,
    pub version_tag: String,
}

impl ProjectInfoConfig {
    /// Load from the primary path, falling back to an alternate path.
    pub fn load_from_primary_or_alt<P: AsRef<Path>, Q: AsRef<Path>>(
        primary: P,
        alternate: Q,
    ) -> Self {
        // Try primary path
        if let Ok(raw) = fs::read_to_string(&primary) {
            return serde_json::from_str(&raw)
                .expect("Failed to parse projectInfo.json from primary path");
        }

        // Try alternate path
        if let Ok(raw) = fs::read_to_string(&alternate) {
            return serde_json::from_str(&raw)
                .expect("Failed to parse projectInfo.json from alternate path");
        }

        panic!(
            "Unable to load projectInfo.json from either:\n  - {}\n  - {}",
            primary.as_ref().display(),
            alternate.as_ref().display()
        );
    }
}
