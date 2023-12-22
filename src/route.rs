use crate::api::category::category_route::category_routes;
use crate::api::items::item_route::item_routes;
use crate::AppState;
use crate::api::user::user_route::user_routes;
use axum::Router;

pub fn config(state: AppState) -> Router {
    Router::new()
        .nest("/category", category_routes(state.clone()))
        .nest("/item", item_routes(state.clone()))
        .nest("/user", user_routes(state.clone()))
}
