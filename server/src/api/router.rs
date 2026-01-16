use crate::{
    api::handlers::{
        cuts::{
            create_cut, delete_cut, get_cut, list_cuts, replace_cut, replace_cut_credits,
            update_cut,
        },
        persons::{
            create_person, delete_person, get_person, list_persons, replace_person, update_person,
        },
        resources::{add_resource, generate_presigned_upload_url},
        search::global_search,
        users::{get_self, get_user, update_self},
        works::{create_work, delete_work, get_work, list_works, replace_work, update_work},
    },
    state::AppState,
};
use salvo::prelude::*;

pub fn app_router(app_state: AppState) -> Router {
    let _ = app_state;
    let router_cuts = Router::with_path("cuts")
        .get(list_cuts)
        .post(create_cut)
        .push(
            Router::with_path("{id}")
                .get(get_cut)
                .patch(update_cut)
                .put(replace_cut)
                .delete(delete_cut)
                .push(Router::with_path("credits").put(replace_cut_credits)),
        );

    let router_persons = Router::with_path("persons")
        .get(list_persons)
        .post(create_person)
        .push(
            Router::with_path("{id}")
                .get(get_person)
                .patch(update_person)
                .put(replace_person)
                .delete(delete_person),
        );

    let router_works = Router::with_path("works")
        .get(list_works)
        .post(create_work)
        .push(
            Router::with_path("{id}")
                .get(get_work)
                .patch(update_work)
                .put(replace_work)
                .delete(delete_work),
        );

    let router_resources = Router::with_path("resources")
        .push(Router::with_path("presign").post(generate_presigned_upload_url))
        .post(add_resource);

    let router_search = Router::with_path("search").get(global_search);

    let router_users = Router::with_path("users")
        .push(Router::with_path("me").get(get_self).patch(update_self))
        .push(Router::with_path("{id}").get(get_user));

    let api_group = Router::new()
        .push(router_cuts)
        .push(router_persons)
        .push(router_works)
        .push(router_resources)
        .push(router_search)
        .push(router_users);

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&api_group);

    // Compose the final "v0" Router
    Router::with_path("v0")
        .push(doc.into_router("docs/openapi.json"))
        // spec url here, so /v0 prefix is necessary
        .push(Scalar::new("/v0/docs/openapi.json").into_router("docs"))
        .push(api_group)
}
