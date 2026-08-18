use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetOperation {
    Buy,
    Sell,
}

impl std::fmt::Display for AssetOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransactionHistory {
    pub operation_type: AssetOperation,
    #[serde(with = "time::serde::iso8601")]
    pub occurred_at: OffsetDateTime,
    pub unit_value: f64,
    pub quantity_bought: f64,
    pub value_delta: f64,
}