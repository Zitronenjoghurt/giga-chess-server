#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct RoomCreationBody {
    #[cfg_attr(
        feature = "validator",
        validate(
            length(min = 3, max = 50),
            custom(function = "crate::validation::alphanumeric::is_alphanumeric")
        )
    )]
    pub name: Option<String>,
    /// If the room is supposed to show up in the public room list.
    pub public: bool,
    /// How much time (in microseconds) each player will get on their clock.
    #[cfg_attr(feature = "validator", validate(range(min = 1000000, max = i64::MAX)))]
    pub time_micros: Option<i64>,
    /// By how much time (in microseconds) the player's clock will be incremented after their move.
    #[cfg_attr(feature = "validator", validate(range(min = 1000000, max = i64::MAX)))]
    pub increment_micros: Option<i64>,
}
