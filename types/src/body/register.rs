#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct RegisterBody {
    pub invite_code: String,
    #[cfg_attr(
        feature = "validator",
        validate(
            length(min = 3, max = 50),
            custom(function = "crate::validation::alphanumeric::is_alphanumeric")
        )
    )]
    pub username: String,
    #[cfg_attr(feature = "validator", validate(length(min = 8, max = 100)))]
    pub password: String,
}
