mod storage;
mod server;

use axum::{Router, extract::State, response::{Html, IntoResponse}, routing::get};
use s3::Bucket;
use std::{error::Error, env, path::Path};

use crate::storage::bucket::generate_bucket;
use crate::storage::project_info::ProjectInfoConfig;
use crate::storage::combined::CombinedProjectSet;
use crate::server::html::render_index;

#[derive(Clone)]
struct AppState {
    bucket: Bucket,
    project_info: ProjectInfoConfig,
    combined: CombinedProjectSet,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bucket = generate_bucket();

    // Primary path: ./config/projectInfo.json
    let primary_path = "./config/projectInfo.json";

    // Alternate path: next to the executable
    let exe_path = env::current_exe().expect("Cannot locate executable path");
    let exe_dir = exe_path.parent().unwrap_or_else(|| Path::new("."));
    let alternate_path = exe_dir.join("projectInfo.json");

    let project_info =
        ProjectInfoConfig::load_from_primary_or_alt(primary_path, &alternate_path);

    let combined = CombinedProjectSet::build(&bucket, &project_info).await;

    let app = Router::new()
        .route("/", get(index))
        .with_state(AppState {
            bucket,
            project_info,
            combined,
        });

    server::core::serve(app, "33031".to_string()).await?;
    Ok(())
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    Html(render_index(&state.combined))
}
