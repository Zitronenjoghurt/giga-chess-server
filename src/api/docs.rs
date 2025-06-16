use crate::api::resources;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title="GigaChess Online",
        description="A giga-chess multiplayer server"
    ),
    paths(
        resources::ping::get_ping
    ),
    tags(
        (name = "Misc", description = "Miscellaneous endpoints")
    ),
    components(
        schemas(),
    )
)]
pub struct ApiDoc;
