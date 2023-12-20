use crate::api::category::category_route::category_routes;
use crate::api::items::item_route::item_routes;
use crate::AppState;
use axum::Router;

pub fn config(state: AppState) -> Router {
    Router::new()
        .nest("/category", category_routes(state.clone()))
        .nest("/item", item_routes(state.clone()))
}
