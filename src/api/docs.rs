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
        routes::room::get_room,
        routes::room::post_room,
        routes::room::join::post_room_join,
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
            body::uuid::UuidBody,
            general::pagination::Pagination,
            query::pagination::PaginationQuery,
            response::message::MessageResponse,
            response::login::LoginResponse,
            response::room_info::PrivateRoomInfo,
            response::room_info::PublicRoomInfo,
            response::room_list::PublicRoomList,
            response::user_info::PublicUserInfo,
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
