#[cfg(feature = "validator")]
pub fn is_alphanumeric(value: &str) -> Result<(), validator::ValidationError> {
    if value.chars().all(|c| c.is_alphanumeric()) {
        Ok(())
    } else {
        Err(validator::ValidationError::new("must be alphanumeric"))
    }
}
