//! Shared domain types. Keep exchange-specific wire models outside this crate.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub id: Uuid,
    pub symbol: String,
    pub side: Side,
    pub strategy_id: String,
    pub created_at: DateTime<Utc>,
    pub entry_reference: Decimal,
    pub stop_price: Decimal,
    pub take_profit_price: Decimal,
    /// Estimated probability calibrated from out-of-sample validation, not a raw score.
    pub win_probability: Option<Decimal>,
}
