use serde::Serialize;
use s3::Bucket;

use crate::storage::project_info::ProjectInfoConfig;

#[derive(Debug, Clone, Serialize)]
pub struct CombinedProjectSet {
    pub projects: Vec<CombinedProject>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CombinedProject {
    pub name: String,
    pub key: String,
    pub description: String,
    pub version_latest: String,
    pub version_changelog: Vec<CombinedVersion>,
    pub files: Vec<DiscoveredFile>,
    pub is_unsorted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CombinedVersion {
    pub version: String,
    pub version_tag: String,
    pub changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DiscoveredFile {
    pub key: String,
    pub relative: String,
    pub url: String,
}

impl CombinedProjectSet {
    pub async fn build(bucket: &Bucket, info: &ProjectInfoConfig) -> Self {
        // 1. Pull all keys from bucket
        let prefix = "smn-get/";
        let list = bucket.list(prefix.to_string(), None).await.unwrap();

        let mut all_files: Vec<DiscoveredFile> = Vec::new();

        for result in list {
            for obj in result.contents {
                let key = obj.key;

                if key.ends_with('/') {
                    continue;
                }

                let relative = key.strip_prefix(prefix).unwrap_or(&key).to_string();
                let url = format!(
                    "https://{}.sfo3.digitaloceanspaces.com/{}",
                    bucket.name(),
                    key
                );

                all_files.push(DiscoveredFile { key, relative, url });
            }
        }

        // 2. Build map for known projects
        let mut projects: Vec<CombinedProject> = info
            .projects
            .iter()
            .map(|p| CombinedProject {
                name: p.name.clone(),
                key: p.key.clone(),
                description: p.description.clone(),
                version_latest: p.version_latest.clone(),
                version_changelog: p
                    .version_changelog
                    .iter()
                    .map(|v| CombinedVersion {
                        version: v.version.clone(),
                        version_tag: v.version_tag.clone(),
                        changes: v.changes.clone(),
                    })
                    .collect(),
                files: Vec::new(),
                is_unsorted: false,
            })
            .collect();

        // 3. Assign files to known projects
        for file in &all_files {
            let mut matched = false;

            for proj in &mut projects {
                if file.key.starts_with(&proj.key) {
                    proj.files.push(file.clone());
                    matched = true;
                    break;
                }
            }

            // 4. If file did not match any known project, it belongs to UNSORTED
            if !matched {
                // Create or append to existing "Unsorted"
                if let Some(unsorted) = projects.iter_mut().find(|p| p.is_unsorted) {
                    unsorted.files.push(file.clone());
                } else {
                    projects.push(CombinedProject {
                        name: "Unsorted".to_string(),
                        key: "unsorted/".to_string(),
                        description: "Files not included in projectInfo.json".to_string(),
                        version_latest: String::new(),
                        version_changelog: Vec::new(),
                        files: vec![file.clone()],
                        is_unsorted: true,
                    });
                }
            }
        }

        Self { projects }
    }
}
