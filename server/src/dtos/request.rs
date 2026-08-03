use salvo::{oapi::ToSchema, prelude::*};
use sea_orm::prelude::Uuid;
use serde::Deserialize;

#[derive(Deserialize, Extractible, Debug, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct PaginationQuery {
    pub pagination_token: Option<String>,
    #[serde(default)]
    pub max_results: Option<u64>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            pagination_token: None,
            max_results: Some(20),
        }
    }
}

#[derive(Deserialize, Extractible, Debug, ToSchema)]
#[salvo(extract(default_source(from = "query")))]
pub struct ExpansionQuery {
    pub expansions: Vec<String>,
}

#[derive(Deserialize, Extractible, Debug, ToSchema)]
pub struct IdRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: Uuid,
    #[salvo(extract(flatten))]
    pub expansions: ExpansionQuery,
}

#[derive(Deserialize, Extractible, Debug, ToSchema)]
pub struct ListRequest {
    pub query: Option<String>,
    #[salvo(extract(flatten))]
    pub pagination: PaginationQuery,
    #[salvo(extract(flatten))]
    pub expansions: ExpansionQuery,
}
