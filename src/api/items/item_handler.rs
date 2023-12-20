use crate::api::items::item_model::{CreateItemModel, ItemModel};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::AppState;

pub async fn get_all_items(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ItemModel, "SELECT * FROM item")
        .fetch_all(&state.db)
        .await;

    
    if query_result.is_err() {
        if query_result.is_err() {
            let err_respone = serde_json::json!({
                "status":"fail",
                "message":"Somthing bad happened while fetching all Items"
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_respone)));
        }
    }
    todo!()
}


pub async fn create_item() {}
pub async fn update_item() {}

pub async fn delete_item() {}

pub async fn get_item_by_id(){}