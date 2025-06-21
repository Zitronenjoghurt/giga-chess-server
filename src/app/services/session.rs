use crate::app::config::Config;
use crate::app::error::{AppError, AppResult};
use crate::app::services::Service;
use crate::database::models::room::Room;
use crate::database::models::session::{NewSession, Session};
use crate::database::stores::session::SessionStore;
use crate::database::stores::user::UserStore;
use crate::database::stores::{Store, Stores};
use giga_chess::prelude::{Engine, Game, PGNMetadata};
use std::sync::Arc;

pub struct SessionService {
    session_store: Arc<SessionStore>,
    user_store: Arc<UserStore>,
}

impl SessionService {
    pub async fn start(&self, engine: &Arc<Engine>, room: &Room) -> AppResult<Session> {
        let Some(white) = (match room.player_white {
            Some(uuid) => self.user_store.find(uuid).await?,
            None => None,
        }) else {
            return Err(AppError::bad_request(
                "Failed to start session: player white not found.",
            ));
        };

        let Some(black) = (match room.player_black {
            Some(uuid) => self.user_store.find(uuid).await?,
            None => None,
        }) else {
            return Err(AppError::bad_request(
                "Failed to start session: player black not found.",
            ));
        };

        let pgn = PGNMetadata::now()
            .site("Giga Chess Online")
            .event("Giga Chess Multiplayer Game")
            .white(&white.name)
            .black(&black.name);

        let game = Game::new(engine, pgn);
        let new_session = NewSession::new(room.id, game, room.time_micros, room.increment_micros);

        self.session_store.create(new_session).await
    }
}

impl Service for SessionService {
    fn initialize(_config: &Arc<Config>, stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            session_store: stores.session.clone(),
            user_store: stores.user.clone(),
        })
    }
}
