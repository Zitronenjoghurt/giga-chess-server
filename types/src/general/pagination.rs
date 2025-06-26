use crate::query::pagination::PaginationQuery;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
