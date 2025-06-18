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
        routes::room::post_room,
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Misc", description = "Miscellaneous endpoints"),
        (name = "Rooms", description = "Room endpoints"),
    ),
    components(
        schemas(
            body::login::LoginBody,
            body::register::RegisterBody,
            body::room_creation::RoomCreationBody,
            response::message::MessageResponse,
            response::login::LoginResponse,
            response::room_creation::RoomCreationResponse,
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
