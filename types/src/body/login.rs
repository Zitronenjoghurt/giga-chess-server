#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct LoginBody {
    #[cfg_attr(
        feature = "validator",
        validate(
            length(min = 3, max = 50),
            custom(function = "crate::validation::alphanumeric::is_alphanumeric")
        )
    )]
    pub username: String,
    pub password: String,
}
