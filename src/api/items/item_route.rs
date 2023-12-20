use crate::{api::items::item_handler, AppState};

use axum::{routing::get, Router};

pub fn item_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(item_handler::get_all_items)
                .post(item_handler::create_item)
                .with_state(app_state.clone()),
        )
        .route(
            "/:id",
            get(item_handler::get_item_by_id)
                .put(item_handler::update_item)
                .delete(item_handler::delete_item)
                .with_state(app_state.clone()),
        )
}
