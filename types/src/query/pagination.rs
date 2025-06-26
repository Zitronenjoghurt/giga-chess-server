#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
#[cfg_attr(feature = "utoipa", into_params(parameter_in = Query))]
#[cfg_attr(feature = "validator", derive(validator::Validate))]
pub struct PaginationQuery {
    #[cfg_attr(feature = "validator", validate(range(min = 1)))]
    page: Option<i64>,
    #[cfg_attr(feature = "validator", validate(range(min = 1, max = 100)))]
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
