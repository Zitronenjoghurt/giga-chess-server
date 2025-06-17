use crate::api::models::*;
use crate::api::routes;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "GigaChess Online",
        description = "A giga-chess multiplayer server"
    ),
    paths(
        routes::login::post_login,
        routes::ping::get_ping,
        routes::register::post_register,
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    components(
        schemas(
            body::login_data::LoginData,
            body::register_data::RegisterData,
            response::message::MessageResponse,
            response::login::LoginResponse
        ),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "BearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
