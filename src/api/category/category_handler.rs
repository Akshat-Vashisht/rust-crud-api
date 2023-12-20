use crate::api::category::category_model::{CategoryModel, CreateCategoryModel};
use crate::AppState;
use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn get_all_categories(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CategoryModel, "SELECT * FROM category")
        .fetch_all(&state.db)
        .await;

    if query_result.is_err() {
        let err_respone = serde_json::json!({
            "status":"fail",
            "message":"Somthing bad happened while fetching all notes"
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err_respone)));
    }

    let category = query_result.unwrap();

    let json_response = serde_json::json!({
        "status":"success",
        "results":category.len(),
        "notes":category
    });

    Ok(Json(json_response))
}

pub async fn get_category_by_id(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM category WHERE category_id = $1",
        id
    )
    .fetch_one(&state.db)
    .await;
    match query_result {
        Ok(category) => {
            let category_response = serde_json::json!({
                "status":"sucess",
                "data":serde_json::json!({
                    "note":category
                })
            });
            return Ok(Json(category_response));
        }
        Err(_) => {
            let err_response = serde_json::json!({
                "status":"failed",
                "message":format!("Category Not Found for {} ID",id)
            });
            return Err((StatusCode::NOT_FOUND, Json(err_response)));
        }
    }
}
pub async fn create_category(
    State(state): State<AppState>,
    Json(body): Json<CreateCategoryModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "INSERT INTO category (category_name) VALUES ($1) RETURNING *",
        body.category_name
    )
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(category) => {
            let response_category = serde_json::json!({
                "status":"success",
                "data":category
            });
            return Ok((StatusCode::CREATED, Json(response_category)));
        }

        Err(e) => {
            if e.to_string()
                .contains("duplicate key violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Category with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }

            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn delete_category(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM category WHERE category_id = $1", id)
        .execute(&state.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_category(
    Path(id): Path<uuid::Uuid>,
    State(state): State<AppState>,
    Json(body): Json<CreateCategoryModel>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CategoryModel,
        "SELECT * FROM category WHERE category_id = $1",
        id
    )
    .fetch_one(&state.db)
    .await;

    if query_result.is_err() {
        let err_response = serde_json::json!({
            "status":"failed",
            "message":format!("Category Not Found for {} ID",id)
        });
        return Err((StatusCode::NOT_FOUND, Json(err_response)));
    }

    let query_result = sqlx::query_as!(
        CategoryModel,
        "UPDATE category SET category_name = $1 WHERE category_id = $2 RETURNING *",
        body.category_name,
        id
    )
    .fetch_one(&state.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}
