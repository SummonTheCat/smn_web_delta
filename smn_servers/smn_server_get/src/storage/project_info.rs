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
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let raw = fs::read_to_string(path).expect("Failed to read projectInfo.json");
        serde_json::from_str(&raw).expect("Failed to parse projectInfo.json")
    }
}
