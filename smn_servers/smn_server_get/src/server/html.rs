use crate::storage::combined::{
    CombinedProject, CombinedProjectSet, CombinedVersion, DiscoveredFile,
};
use std::collections::BTreeMap;

pub fn render_index(set: &CombinedProjectSet) -> String {
    let categories = categorize_projects(set);

    let mut html = String::new();

    html.push_str(
        r#"
<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<title>Projects</title>
<style>
    body {
        font-family: sans-serif;
        margin: 40px;
        background: #fafafa;
    }
    details {
        margin-bottom: 12px;
        padding: 8px 12px;
        background: #fff;
        border-radius: 6px;
        border: 1px solid #ddd;
    }
    summary {
        cursor: pointer;
        font-size: 1.25em;
        padding: 4px;
    }
    .project-entry {
        margin-left: 15px;
        padding: 10px 14px;
        background: #f7f7f7;
        border-left: 2px solid #ccc;
        border-radius: 4px;
        margin-bottom: 12px;
    }
    .project-desc {
        color: #444;
        margin: 6px 0 12px 0;
    }
    .group {
        margin-left: 15px;
        padding: 6px 10px;
        background: #fafafa;
        border-left: 2px solid #bbb;
        border-radius: 4px;
        margin-bottom: 10px;
    }
    .version-block {
        margin-left: 20px;
        padding: 6px;
        border-left: 2px solid #aaa;
    }
    ul {
        margin: 6px 0 12px 0;
        padding-left: 20px;
    }
    .version-row {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    padding: 10px 0;
    border-bottom: 1px solid #ddd;
    border-top: 1px solid #ddd;
}

.version-left {
    display: flex;
    min-width: 30vw;
    flex: 0 0 auto;
    align-items: center;
}

.version-right {
    display: flex;
    flex: 1;
    align-items: center;
}

    .version-tag {
        background: #eee;
        padding: 4px 8px;
        margin-right: 12px;
        margin-left: 12px;
        border-radius: 4px;
        font-size: 0.9em;
        color: #333;
        border: 1px solid #ccc;
    }

    .changelog-list {
        background: #eee;
        padding: 10px;
        padding-left: 20px;
        border-radius: 4px;
        border: 1px solid #ccc;
    }
</style>
</head>
<body>
<h1>Projects</h1>
"#,
    );

    for (category, projects) in categories {
        html.push_str("<details>");
        html.push_str(&format!("<summary>{}</summary>", category));

        for proj in projects {
            html.push_str(&render_project(proj));
        }

        html.push_str("</details>");
    }

    html.push_str("</body></html>");
    html
}

fn categorize_projects<'a>(
    set: &'a CombinedProjectSet,
) -> BTreeMap<String, Vec<&'a CombinedProject>> {
    let mut map: BTreeMap<String, Vec<&CombinedProject>> = BTreeMap::new();

    for proj in &set.projects {
        // key example: "smn-get/games/nullmarker/"
        let parts: Vec<&str> = proj.key.split('/').collect();
        let category = if parts.len() > 2 {
            parts[1].to_string() + "/"
        } else {
            "misc/".to_string()
        };

        map.entry(category).or_default().push(proj);
    }

    map
}

fn render_project(proj: &CombinedProject) -> String {
    let mut out = String::new();

    out.push_str("<div class=\"project-entry\">");
    out.push_str(&format!("<h2>{}</h2>", proj.name));
    out.push_str(&format!(
        "<div class=\"project-desc\">{}</div>",
        proj.description
    ));

    let version_groups = group_files_by_version(&proj.files);

    for (vg, files) in version_groups {
        out.push_str("<details class=\"group\">");
        out.push_str(&format!("<summary>{}</summary>", vg));

        for f in &files {
            let filename = f.relative.split('/').last().unwrap_or(&f.relative);

            if let Some(log) = proj
                .version_changelog
                .iter()
                .find(|v| v.version.split('/').last().unwrap_or(&v.version) == filename)
            {
                out.push_str(&render_version_block(log, f));
            } else {
                out.push_str(&render_version_block_no_log(f));
            }
        }

        out.push_str("</details>");
    }

    out.push_str("</div>");
    out
}

fn group_files_by_version(files: &[DiscoveredFile]) -> BTreeMap<String, Vec<DiscoveredFile>> {
    let mut map = BTreeMap::<String, Vec<DiscoveredFile>>::new();

    for f in files {
        let parts: Vec<&str> = f.relative.split('/').collect();
        let group = if parts.len() > 2 {
            format!("{}/", parts[parts.len() - 2])
        } else {
            "misc/".to_string()
        };

        map.entry(group).or_default().push(f.clone());
    }

    // sort each group newest → oldest
    for (_key, list) in map.iter_mut() {
        list.sort_by(|a, b| b.relative.cmp(&a.relative));
    }

    map
}

fn render_version_block(v: &CombinedVersion, f: &DiscoveredFile) -> String {
    let mut out = String::new();

    out.push_str(r#"<div class="version-row">"#);

    // LEFT COLUMN → the FILE LINK ONLY
    out.push_str(&format!(
        r#"<div class="version-left"><a href="{0}">{1}</a></div>"#,
        f.url, f.relative
    ));

    // RIGHT COLUMN → version tag + changelog (NO filename)
    out.push_str(r#"<div class="version-right">"#);
    out.push_str(&format!("<b class=\"version-tag\">{}</b>", v.version_tag));

    if !v.changes.is_empty() {
        out.push_str("<ul class=\"changelog-list\">");
        for c in &v.changes {
            out.push_str(&format!("<li>{}</li>", c));
        }
        out.push_str("</ul>");
    }

    out.push_str("</div></div>");
    out
}

fn render_version_block_no_log(f: &DiscoveredFile) -> String {
    format!(
        r#"<div class="version-row">
            <div class="version-left">
                <a href="{0}">{1}</a>
            </div>
            <div class="version-right">
                <b>No changelog</b>
            </div>
        </div>"#,
        f.url, f.relative
    )
}
