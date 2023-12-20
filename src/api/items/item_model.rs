use serde::{Deserialize, Serialize};

//  use sqlx::types::BigDecimal;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]

pub struct ItemModel {
    pub item_id: Uuid,
    pub item_name: String,
    pub item_price: f64,
    pub item_qty: i16,
    pub category_id: Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateItemModel {
    pub item_name: String,
    pub item_price: f64,
    pub item_qty: i16,
    pub category_id: Uuid,
}
