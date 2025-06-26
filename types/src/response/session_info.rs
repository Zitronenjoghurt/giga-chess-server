#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for PrivateSessionInfo {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
