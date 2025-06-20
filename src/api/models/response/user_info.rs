use crate::database::models::user::User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PublicUserInfo {
    pub name: String,
}

impl PublicUserInfo {
    pub fn from_user(user: &User) -> Self {
        Self {
            name: user.name.clone(),
        }
    }
}
