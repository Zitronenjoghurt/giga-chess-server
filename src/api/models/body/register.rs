use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct RegisterBody {
    pub invite_code: String,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(length(min = 8, max = 100))]
    pub password: String,
}
