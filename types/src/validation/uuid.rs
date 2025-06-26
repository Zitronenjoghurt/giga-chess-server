#[cfg(feature = "uuid")]
#[cfg(feature = "validator")]
pub fn is_uuid(value: &str) -> Result<(), validator::ValidationError> {
    if uuid::Uuid::parse_str(value).is_ok() {
        Ok(())
    } else {
        Err(validator::ValidationError::new("must be a valid UUID"))
    }
}
