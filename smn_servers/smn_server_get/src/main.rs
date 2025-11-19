use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::{BehaviorVersion, Region};
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use std::{error::Error, net::SocketAddr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let access_key = std::env::var("DO_SPACES_KEY")?;
    let secret_key = std::env::var("DO_SPACES_SECRET")?;
    let endpoint = "https://sfo3.digitaloceanspaces.com";
    let region = "sfo3";
    let bucket = "smn-site-storage";

    let creds = Credentials::new(access_key, secret_key, None, None, "env");

    let config = aws_sdk_s3::Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(region.to_owned()))
        .endpoint_url(endpoint)
        .force_path_style(true)
        .credentials_provider(creds)
        .build();

    let client = Client::from_conf(config);

    let app = Router::new().route("/", get(index)).with_state(AppState {
        client,
        bucket: bucket.to_owned(),
    });

    let addr: SocketAddr = "127.0.0.1:3001".parse()?;

    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    client: Client,
    bucket: String,
}

async fn index(State(state): State<AppState>) -> impl IntoResponse {
    let prefix = "smn-get/";

    let resp = state
        .client
        .list_objects_v2()
        .bucket(&state.bucket)
        .prefix(prefix)
        .send()
        .await
        .unwrap();

    let mut html = String::new();
    html.push_str("<h1>Downloads</h1><ul>");

    for obj in resp.contents() {
        let key = obj.key().unwrap_or("");
        if key.ends_with('/') {
            continue;
        }

        // Keep the FULL relative path under smn-get/
        let relative = key.trim_start_matches(prefix);

        // Correct public URL
        let url = format!(
            "https://{}.sfo3.digitaloceanspaces.com/{}",
            state.bucket, key
        );

        html.push_str(&format!("<li><a href=\"{}\">{}</a></li>", url, relative));
    }

    html.push_str("</ul>");

    Html(html)
}
