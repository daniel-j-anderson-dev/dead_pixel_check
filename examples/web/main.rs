use axum::Router;
use color_eyre::Report;
use tokio::net::TcpListener;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() -> Result<(), Report> {
    let ip = "localhost:8080";
    let listener = TcpListener::bind(ip).await?;
    let router = Router::new().nest_service(
        "/",
        ServeFile::new("examples/web/assets/index.html"),
    );
    println!("serving on http://{}/index.html", ip);
    axum::serve(listener, router).await?;
    return Ok(());
}
