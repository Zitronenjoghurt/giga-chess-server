use crate::app::error::{AppError, AppResult};
use crate::database::models::session::Session;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct PrivateSessionInfo {
    pub uuid: String,
    pub room_uuid: String,
    /// The giga-chess Game as a serialized byte array.
    pub game: Vec<u8>,
    /// How many microseconds are left on the clock for white.
    pub white_time_left_micros: Option<i64>,
    /// How many microseconds are left on the clock for black.
    pub black_time_left_micros: Option<i64>,
    /// How many microseconds to increment a player's clock by after their move.
    pub increment_micros: Option<i64>,
    /// The timestamp of the last move in microseconds.
    pub last_move: i64,
}

impl PrivateSessionInfo {
    pub fn from_session(session: &Session) -> AppResult<Self> {
        let game = session
            .game
            .serialize()
            .map_err(|e| AppError::Serialization(e.to_string()))?;

        Ok(Self {
            uuid: session.id.to_string(),
            room_uuid: session.room_id.to_string(),
            game,
            white_time_left_micros: session.time_left_white(),
            black_time_left_micros: session.time_left_black(),
            increment_micros: session.increment_micros,
            last_move: session.last_move.timestamp_micros(),
        })
    }
}
