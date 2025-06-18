use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct LoginBody {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    pub password: String,
}
