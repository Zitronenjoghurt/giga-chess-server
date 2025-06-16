use crate::api::*;
use axum::Router;
use state::AppState;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

pub mod error;
pub mod state;

pub fn build_app() -> Router {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let state = AppState::initialize(&database_url);

    Router::<AppState>::new()
        .merge(resources::ping::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(state)
}
