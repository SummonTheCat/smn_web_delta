use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use s3::Bucket;
use s3::creds::Credentials;
use s3::Region;
use std::{error::Error, net::SocketAddr};

#[derive(Clone)]
struct AppState {
    bucket: Bucket,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load DO Spaces credentials
    let access_key = std::env::var("DO_SPACES_KEY")?;
    let secret_key = std::env::var("DO_SPACES_SECRET")?;

    // DO Spaces config
    let bucket_name = "smn-site-storage";
    let region_name = "sfo3";
    let endpoint = "https://sfo3.digitaloceanspaces.com";

    // Credentials
    let creds = Credentials::new(
        Some(&access_key),
        Some(&secret_key),
        None,
        None,
        None,
    )?;

    // Construct Region
    let region = Region::Custom {
        region: region_name.to_string(),
        endpoint: endpoint.to_string(),
    };

    // Initialize bucket client
    let bucket = Bucket::new(bucket_name, region, creds)?.with_path_style(); 
    // IMPORTANT: DO Spaces requires path-style for many operations

    // Build Axum app
    let app = Router::new()
        .route("/", get(index))
        .with_state(AppState { bucket: *bucket });

    // Start server
    let addr: SocketAddr = "127.0.0.1:3001".parse()?;
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let prefix = "smn-get/";

    // List ALL objects under prefix recursively
    let results = state
        .bucket
        .list(prefix.to_string(), None)
        .await
        .unwrap();

    println!("Found {} results", results.len());

    let mut html = String::new();
    html.push_str("<h1>Downloads</h1><ul>");

    for result in results {
        println!("Processing result with {} contents", result.contents.len());

        for obj in result.contents {
            let key = obj.key;

            println!("Found object with key: {}", key);

            if key.ends_with('/') {
                continue;
            }

            let relative = key.strip_prefix(prefix).unwrap_or(&key);

            let url = format!(
                "https://{}.sfo3.digitaloceanspaces.com/{}",
                state.bucket.name(),
                key
            );

            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                url, relative
            ));
        }
    }

    html.push_str("</ul>");
    Html(html)
}
