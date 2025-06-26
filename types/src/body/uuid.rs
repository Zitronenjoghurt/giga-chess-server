#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
#[cfg(feature = "uuid")]
pub struct UuidBody {
    #[cfg_attr(
        feature = "validator",
        validate(custom(function = "crate::validation::uuid::is_uuid"))
    )]
    pub uuid: String,
}

#[cfg(feature = "uuid")]
impl UuidBody {
    pub fn get_uuid(&self) -> uuid::Uuid {
        uuid::Uuid::parse_str(&self.uuid).unwrap()
    }
}
