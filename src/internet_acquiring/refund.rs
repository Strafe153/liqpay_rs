use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct RefundRequest {
    version: Version,
    action: Action,
    public_key: String,
    order_id: String,
    amount: f64,
}

impl RefundRequest {
    pub fn new(public_key: impl Into<String>, order_id: String, amount: f64) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Refund,
            public_key: public_key.into(),
            order_id,
            amount,
        }
    }
}

impl LiqPayRequest<RefundResponse, Sha3_256> for RefundRequest {}

#[derive(Deserialize, Debug)]
pub struct RefundResponse {
    pub result: Result,
    pub status: Status,
    pub wait_amount: Option<bool>,
    pub action: Option<Action>,
    pub payment_id: Option<u32>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RefundResponse {}
