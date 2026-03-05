use salvo::oapi::ToSchema;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, ToSchema)]
pub struct PaginationQuery {
    pub cursor: Option<String>,
    pub limit: Option<u64>,
}
