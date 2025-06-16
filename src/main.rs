use crate::app::build_app;
use std::io;

mod api;
mod app;
mod database;

#[tokio::main]
async fn main() -> io::Result<()> {
    let app = build_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8869").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
