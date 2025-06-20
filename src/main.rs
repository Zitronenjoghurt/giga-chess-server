use giga_chess_server::app::build_app;
use giga_chess_server::app::state::AppState;
use std::io;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();

    let file_appender = tracing_appender::rolling::daily("logs", "server.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking_appender)
                .with_ansi(false),
        )
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let state = AppState::initialize(&database_url);
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8869").await?;
    info!("Server now listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
