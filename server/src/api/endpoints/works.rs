use crate::dtos::request::PaginationQuery;
use crate::error::AppResult;
use crate::state::AppState;
use salvo::oapi::endpoint;
use salvo::oapi::extract::QueryParam;
use salvo::prelude::*;

#[endpoint]
pub async fn list_works(
    res: &mut Response,
    depot: &mut Depot,
    query: QueryParam<PaginationQuery, false>,
) -> AppResult<()> {
    let app_state = depot.obtain::<AppState>().unwrap();
    let db = app_state.db.clone();

    // Use into_inner() to get the PaginationQuery from QueryParam
    let works_response =
        crate::services::works::list_works(&db, query.into_inner().unwrap_or_default()).await?;

    res.render(works_response);

    Ok(())
}

#[endpoint]
pub async fn get_work() {
    todo!()
}

#[endpoint]
pub async fn create_work() {
    todo!()
}

#[endpoint]
pub async fn update_work() {
    todo!()
}

#[endpoint]
pub async fn delete_work() {
    todo!()
}

#[endpoint]
pub async fn replace_work() {
    todo!()
}
