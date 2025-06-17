use crate::app::error::{AppError, AppResult};
use crate::app::security::hash_bytes;
use crate::app::services::Service;
use crate::database::models::user::{NewUser, User};
use crate::database::stores::invite_code::InviteCodeStore;
use crate::database::stores::user::UserStore;
use crate::database::stores::{Store, Stores};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    invite_code_store: Arc<InviteCodeStore>,
    user_store: Arc<UserStore>,
}

impl Service for UserService {
    fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            invite_code_store: stores.invite_code.clone(),
            user_store: stores.user.clone(),
        })
    }
}

impl UserService {
    pub fn register(
        &self,
        invite_code_string: &str,
        name: &str,
        password: &str,
    ) -> AppResult<User> {
        let Ok(invite_id) = Uuid::parse_str(invite_code_string) else {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        let Some(mut invite_code) = self.invite_code_store.find(invite_id)? else {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        if invite_code.used {
            return Err(AppError::InvalidInput("Invalid invite code".to_string()));
        };

        invite_code.used = true;
        let invite_code = self.invite_code_store.save(invite_code)?;
        let password_hash = hash_bytes(password.as_bytes())?;

        let new_user = NewUser::new(name, invite_code.id, &password_hash);
        self.user_store.create(new_user)
    }
}
