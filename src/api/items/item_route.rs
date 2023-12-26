use crate::{api::items::item_handler, jwtauth::auth, AppState};

use axum::{middleware, routing::get, Router};

pub fn item_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(item_handler::get_all_items).post(item_handler::create_item),
        )
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .route(
            "/:id",
            get(item_handler::get_item_by_id)
                .put(item_handler::update_item)
                .delete(item_handler::delete_item),
        )
        .route_layer(middleware::from_fn_with_state(app_state.clone(), auth))
        .with_state(app_state.clone())
}
