use crate::database::models::Model;
use crate::database::types::serialized_game::SerializedGame;
use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use giga_chess::prelude::{Color, Game};
use uuid::Uuid;

#[derive(Debug, Identifiable, Queryable, Selectable, AsChangeset, Clone, PartialEq, Eq)]
#[diesel(table_name = crate::database::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: Uuid,
    pub room_id: Uuid,
    pub game: SerializedGame,
    /// How many microseconds white had left after the last move.
    pub white_timer_micros: Option<i64>,
    /// How many microseconds black had left after the last move.
    pub black_timer_micros: Option<i64>,
    /// By how many microseconds the timer is incremented after each move.
    pub increment_micros: Option<i64>,
    /// The timestamp at which the last move was played.
    pub last_move: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn time_left(&self, color: Color) -> Option<i64> {
        let timer = match color {
            Color::White => self.white_timer_micros,
            Color::Black => self.black_timer_micros,
        }?;

        Some(if self.game.0.side_to_move() == color {
            let time_passed = (Utc::now() - self.last_move).num_seconds();
            timer - time_passed
        } else {
            timer
        })
    }

    pub fn time_left_white(&self) -> Option<i64> {
        self.time_left(Color::White)
    }

    pub fn time_left_black(&self) -> Option<i64> {
        self.time_left(Color::Black)
    }
}

impl Model for Session {
    type NewModel = NewSession;
    type PrimaryKeyType = Uuid;
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::sessions)]
pub struct NewSession {
    pub id: Uuid,
    pub room_id: Uuid,
    pub game: SerializedGame,
    pub white_timer_micros: Option<i64>,
    pub black_timer_micros: Option<i64>,
    pub increment_micros: Option<i64>,
}

impl NewSession {
    pub fn new(
        room_id: Uuid,
        game: Game,
        time_micros: Option<i64>,
        increment_micros: Option<i64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            room_id,
            game: SerializedGame(game),
            white_timer_micros: time_micros,
            black_timer_micros: time_micros,
            increment_micros,
        }
    }
}
