use crate::api::models::query::pagination::PaginationQuery;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Pagination {
    pub results: i64,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

impl Pagination {
    pub fn from_query(query: &PaginationQuery, results: i64, total: i64) -> Self {
        Self {
            results,
            total,
            page: query.page(),
            limit: query.limit(),
        }
    }
}
