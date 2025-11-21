mod storage;
mod server;

use axum::{Router, extract::State, response::{Html, IntoResponse}, routing::get};
use s3::Bucket;
use std::error::Error;

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
    let project_info = ProjectInfoConfig::load_from_file("./config/projectInfo.json");

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
