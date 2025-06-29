use crate::app::config::Config;
use crate::app::error::{AppError, AppResult};
use crate::app::security::{generate_jwt, hash_bytes, verify_bytes};
use crate::app::services::Service;
use crate::database::models::user::{NewUser, User};
use crate::database::stores::invite_code::InviteCodeStore;
use crate::database::stores::user::UserStore;
use crate::database::stores::{Store, Stores};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    config: Arc<Config>,
    invite_code_store: Arc<InviteCodeStore>,
    user_store: Arc<UserStore>,
}

impl Service for UserService {
    fn initialize(config: &Arc<Config>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            config: config.clone(),
            invite_code_store: stores.invite_code.clone(),
            user_store: stores.user.clone(),
        })
    }
}

impl UserService {
    pub async fn register(
        &self,
        invite_code_string: &str,
        name: &str,
        password: &str,
    ) -> AppResult<User> {
        let Ok(invite_id) = Uuid::parse_str(invite_code_string) else {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        let Some(mut invite_code) = self.invite_code_store.find(invite_id).await? else {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        if invite_code.used {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        let existing_user = self.user_store.find_by_name(name).await?;
        if existing_user.is_some() {
            return Err(AppError::already_exists("Username"));
        }

        invite_code.used = true;
        let invite_code = self.invite_code_store.save(invite_code).await?;
        let password_hash = hash_bytes(password.as_bytes())?;

        let new_user = NewUser::new(name, invite_code.id, &password_hash);
        self.user_store.create(new_user).await
    }

    pub async fn login(&self, username: &str, password: &str) -> AppResult<String> {
        let Some(user) = self.user_store.find_by_name(username).await? else {
            return Err(AppError::InvalidCredentials);
        };

        if !verify_bytes(password.as_bytes(), &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        let Ok(jwt) = generate_jwt(
            &user,
            &self.config.jwt_key,
            Duration::from_secs(self.config.jwt_token_ttl_sec),
        ) else {
            return Err(AppError::InvalidCredentials);
        };

        Ok(jwt)
    }
}
