use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UuidBody {
    pub uuid: String,
}

impl UuidBody {
    pub fn get_uuid(&self) -> Uuid {
        Uuid::parse_str(&self.uuid).unwrap()
    }
}

impl Validate for UuidBody {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if Uuid::parse_str(&self.uuid).is_err() {
            let mut error = ValidationError::new("invalid_uuid");
            error.message = Some("The provided UUID is not valid".into());
            errors.add("uuid", error);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
