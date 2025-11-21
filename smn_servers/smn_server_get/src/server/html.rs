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
"#,
    );

    html.push_str(&render_font());

    html.push_str(&render_style());

    let mut header_content = String::new();

    header_content.push_str("</head><body><h1 class=\"title\">SmnGet</h1>");
    header_content.push_str(
        "<p class=\"subtitle\">Official file repository of downloads provided by SummonBox Studio</p>",
    );

    // Place header under a div for styling
    html.push_str(&format!(
        r#"<div class="header-container">{}</div>"#,
        header_content
    ));

    let mut detail_content = String::new();

    for (category, projects) in categories {
        detail_content.push_str("<details class=\"category\">");
        detail_content.push_str(&format!("<summary>{}</summary>", category));

        let mut project_content = String::new();

        for proj in projects {
            project_content.push_str(&render_project(proj));
        }

        // Place projects under a div for styling
        detail_content.push_str(&format!(
            r#"<div class="projects-container">{}</div>"#,
            project_content
        ));

        detail_content.push_str("</details>");
    }

    // Place details under a div for styling
    html.push_str(&format!(
        r#"<div class="details-container">{}</div>"#,
        detail_content
    ));

    html.push_str("</body></html>");
    html
}

fn render_font() -> String {
    r#"
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Google+Sans+Code:ital,wght@0,300..800;1,300..800&family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap" rel="stylesheet">
    "#.to_string()
}

fn render_style() -> String {
    r#"
<style>
:root {
    /* Text Colors */
    --color-text-01: #ffffff;
    --color-text-02: #e0e0e0;
    --color-text-03: #b0b0b0;
    --color-text-04: #808080;

    /* Primary Colors */
    --color-primary-01: #4e9cff;
    --color-primary-02: #1f7bff;
    --color-primary-03: #005de0;
    --color-primary-04: #003a99;

    /* Accent Colors */
    --color-accent-01: #ffcf53;
    --color-accent-02: #ffb720;
    --color-accent-03: #e28d00;
    --color-accent-04: #a46400;

    /* Background Colors */
    --color-background-01: #0b0c10;
    --color-background-02: #16181e;
    --color-background-03: #1e2128;
    --color-background-04: #252931;

    /* Typography */
    --font-family-01: "Nunito" , Arial, sans-serif;
    --font-family-02: "Google Sans Code", monospace, monospace;
    --font-size-01: 16px;
    --font-size-02: 20px;
    --font-size-03: 25px;

    /* Shared Layout Variables */
    --radius-01: 5px;
    --radius-02: 8px;

    --space-01: 5px;
    --space-02: 10px;
    --space-03: 20px;

    --border-01: 1px solid var(--color-background-04);
    --border-02: 1px solid var(--color-background-03);
}

* {
    margin: 0;
    padding: 0;
}

html, body {
    background-color: var(--color-background-01);
    color: var(--color-text-01);
    font-family: var(--font-family-01);
    font-size: var(--font-size-01);
}

summary {
    cursor: pointer;
    font-weight: bold;
    padding: var(--space-02);
    background: var(--color-background-03);
    border-radius: var(--radius-01);
    user-select: none;
    transition: background 0.25s ease, transform 0.2s ease;
}

summary:hover {
    background: var(--color-background-04);
}

summary:active {
    transform: scale(0.98);
}

/* Smooth accordion animation */
details[open] > summary {
    border-bottom-left-radius: 0;
    border-bottom-right-radius: 0;
}

details.category,
details.group {
    overflow: hidden;
    transition: background 0.3s ease, border-color 0.3s ease;
}

details.group {
    border: var(--border-01);
    border-radius: var(--radius-01);
}

/* Project entries */
.project-entry {
    padding: var(--space-02);
    margin-bottom: var(--space-02);
    background: var(--color-background-02);
    border: var(--border-01);
    border-radius: var(--radius-01);
    box-shadow: 0px 0px 4px rgba(0,0,0,0.25);
    transition: transform 0.15s ease, box-shadow 0.2s ease, border-color 0.15s ease;
}

.project-entry:hover {
    box-shadow: 0px 4px 8px rgba(0,0,0,0.30);
    border-color: var(--color-background-01);
}

.projects-container {
    padding: var(--space-02);
    border-radius: var(--radius-01);
}

.version-row {
    padding: var(--space-01) 0;
    border-bottom: var(--border-02);
    display: grid;
    grid-template-columns: 400px auto;
    transition: background 0.25s ease;
}

.version-row:hover {
    background: rgba(255,255,255,0.03);
}

.version-left {
    display: flex;
    align-items: center;
}

.header-container {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-02);
    background: var(--color-background-04);
    box-shadow: 0px 2px 6px rgba(0,0,0,0.3);
}

.category {
    margin: var(--space-02);
    border: var(--border-01);
    border-radius: var(--radius-01);
    transition: border-color 0.15s ease;
}

.category:hover {
    border-color: var(--color-background-03);
}

.title {
    font-size: var(--font-size-03);
    font-weight: bold;
    letter-spacing: 0.5px;
}

/* VERSION DOWNLOAD BUTTON */
.version-link {
    color: var(--color-primary-02);
    text-decoration: none;
    margin-left: var(--space-03);
    background: var(--color-background-01);
    padding: var(--space-01);
    border: 1px solid var(--color-primary-02);
    border-radius: var(--radius-01);
    transition: border-color 0.15s ease, color 0.15s ease, transform 0.15s ease;
}

.version-link:hover {
    border-color: var(--color-primary-01);
    color: var(--color-primary-01);
    transform: translateX(2px);
}

.version-link:active {
    border-color: var(--color-primary-04);
    color: var(--color-primary-04);
    transform: translateX(0px);
}

.space-01 {
    height: var(--space-01);
    width: var(--space-01);
}

.space-02 {
    height: var(--space-02);
    width: var(--space-02);
}
    

</style>
"#
    .to_string()
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
    out.push_str("<div class=\"space-01\"> </div>");
    out.push_str(&format!(
        "<div class=\"project-desc\">{}</div>",
        proj.description
    ));
    out.push_str("<div class=\"space-02\"> </div>");

    let version_groups = group_files_by_version(&proj.files);

    for (vg, files) in version_groups {
        out.push_str("<details class=\"group\">");
        out.push_str(&format!("<summary>{}</summary>", vg));
        out.push_str("<div class=\"space-01\"> </div>");

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
    let link_visual_text = f.relative.split('/').last().unwrap_or(&f.relative);

    out.push_str(&format!(
        r#"<div class="version-left"><a href="{0}" class="version-link">{1}</a></div>"#,
        f.url, link_visual_text
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
