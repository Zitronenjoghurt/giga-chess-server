use crate::api::*;
use axum::extract::connect_info::IntoMakeServiceWithConnectInfo;
use axum::Router;
use state::AppState;
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod config;
pub mod error;
pub mod security;
mod services;
pub mod state;

pub fn build_app(state: AppState) -> IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    Router::<AppState>::new()
        .merge(routes::login::router())
        .merge(routes::ping::router())
        .merge(routes::register::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>()
}
