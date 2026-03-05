use sea_orm::{DatabaseConnection, EntityTrait};

use crate::dtos::request::PaginationQuery;
use crate::dtos::response::{Meta, ResponseData};
use crate::entities::prelude::*;
use crate::error::AppResult;

pub async fn list_works(
    db: &DatabaseConnection,
    query: PaginationQuery,
) -> AppResult<ResponseData<Vec<crate::entities::works::Model>>> {
    // TODO: Implement actual cursor-based pagination using `query.cursor` and `query.limit`
    let works = Works::find().all(db).await?;

    let meta = Meta {
        next_token: None, // Placeholder: implement actual token logic
        previous_token: None,
    };

    Ok(ResponseData::new(works).with_meta(meta))
}
