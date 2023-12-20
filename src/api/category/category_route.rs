use crate::{api::category::category_handler, AppState};
use axum::{routing::get, Router};

pub fn category_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(category_handler::get_all_categories)
                .post(category_handler::create_category)
                .with_state(app_state.clone()),
        )
        .route(
            "/:id",
            get(category_handler::get_category_by_id)
                .put(category_handler::update_category)
                .delete(category_handler::delete_category)
                .with_state(app_state.clone()),
        )
}
