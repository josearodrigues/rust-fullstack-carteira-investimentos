use crate::models::purchase_history::PurchaseHistory;
use serde::Serialize;
use sqlx::types::Json;

#[derive(Serialize, sqlx::FromRow)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<PurchaseHistory>>,
}
