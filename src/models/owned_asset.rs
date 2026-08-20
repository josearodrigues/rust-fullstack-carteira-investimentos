use crate::models::{
    transaction_history::TransactionHistory,
    portfolio_summary::format_brl
};
use serde::Serialize;
use sqlx::types::Json;

#[derive(Serialize, sqlx::FromRow)]
pub struct OwnedAsset {
    pub id: i64,
    pub name: String,
    pub unit_value: f64,
    pub value_delta: f64,
    pub quantity_owned: f64,
    pub purchase_history: Json<Vec<TransactionHistory>>,
}

impl OwnedAsset {
    pub fn invest_value(&self) -> String {
        format_brl(self.unit_value * self.quantity_owned)        
    }
}
