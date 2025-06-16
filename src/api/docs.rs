use crate::api::resources;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
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
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "UsernameAuth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Username"))),
            );
            components.add_security_scheme(
                "TokenAuth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Token"))),
            );
        }
    }
}
