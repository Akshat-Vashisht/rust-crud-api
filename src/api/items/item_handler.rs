use crate::api::items::item_model::{CreateItemModel, ItemModel};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

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

    let items = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": items.len(),
        "notes": items
    });
    Ok(Json(json_response))
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(body): Json<CreateItemModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ItemModel,"INSERT INTO item (item_name,item_price,item_qty,category_id) VALUES ($1,$2,$3,$4) RETURNING *",body.item_name,body.item_price,body.item_qty,body.category_id)
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(item) => {
            let response = serde_json::json!({
                "status": "success",
                "data":item
            });
            return Ok((StatusCode::CREATED, Json(response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key violates unique constraint")
            {
                let response = serde_json::json!({
                    "status":"fail",
                    "message":"Item with that name already exists"
                });
                return Err((StatusCode::CONFLICT, Json(response)));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn update_item(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    Json(body): Json<ItemModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ItemModel, "SELECT * FROM item WHERE item_id = $1", id)
        .fetch_one(&state.db)
        .await;

    if query_result.is_err() {
        let response = serde_json::json!({
            "status":"failed",
            "message":format!("Item Not Found for {} ID",id)
        });
        return Err((StatusCode::NOT_FOUND, Json(response)));
    }

    let query_result = sqlx::query_as!(
        ItemModel,
        "UPDATE item SET item_name = $1,item_price = $2,item_qty = $3, category_id = $4 WHERE item_id = $5 RETURNING *",
        body.item_name,
        body.item_price,
        body.item_qty,
        body.category_id,
        id
    )
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(item) => {
            let response = serde_json::json!({"status": "success","data": serde_json::json!({
                "item": item
            })});

            return Ok(Json(response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_item(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM item WHERE item_id = $1", id)
        .execute(&state.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Item with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_item_by_id(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ItemModel, "SELECT * FROM item WHERE item_id = $1", id)
        .fetch_one(&state.db)
        .await;

    match query_result {
        Ok(item) => {
            let response = serde_json::json!({
                "status":"sucess",
                "data":serde_json::json!({
                    "note":item
                })
            });
            return Ok(Json(response));
        }
        Err(_) => {
            let err_response = serde_json::json!({
                "status":"failed",
                "message":format!("Item Not Found for {} ID",id)
            });
            return Err((StatusCode::NOT_FOUND, Json(err_response)));
        }
    }
}
