use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct PaginationQuery {
    #[validate(range(min = 1))]
    page: Option<i64>,
    #[validate(range(min = 1, max = 100))]
    limit: Option<i64>,
}

impl PaginationQuery {
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(10)
    }
}
