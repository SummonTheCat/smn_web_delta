use axum::Router;
use std::{error::Error, net::SocketAddr};

pub async fn serve(app: Router, port: String) -> Result<(), Box<dyn Error>> {
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
