use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct CategoryModel {
    pub category_id: Uuid,
    pub category_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateCategoryModel {
    pub category_name: String,
}
