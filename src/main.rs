use giga_chess_server::app::build_app;
use giga_chess_server::app::state::AppState;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let state = AppState::initialize(&database_url);
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8869").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
