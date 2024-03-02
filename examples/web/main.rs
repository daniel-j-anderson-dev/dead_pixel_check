use std::process::ExitStatus;
use axum::Router;
use color_eyre::{eyre::eyre, Report};
use tokio::{net::TcpListener, process::Command};
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> Result<(), Report> {
    build_wasm().await?;
    
    let listener = TcpListener::bind("[::1]:8080").await?;
    let router = Router::new().nest_service("/", ServeFile::new("examples/web/index.html"));

    println!("serving on http://localhost:8080/index.html");
    axum::serve(listener, router).await?;

    return Ok(());
}

async fn build_wasm() -> Result<ExitStatus, Report> {
    let build_status = Command::new("cargo")
        .current_dir("./")
        .args(&["build", "--release", "--target=wasm32-unknown-unknown"])
        .status()
        .await?;
    if !build_status.success() {
        return Err(eyre!("Failed to build wasm\n{}", build_status));
    }
    return Ok(build_status);
}
