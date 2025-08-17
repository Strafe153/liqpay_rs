use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to initiate a refund operation.
#[derive(Debug, Serialize)]
pub struct RefundRequest {
    version: Version,
    action: Action,
    public_key: String,
    order_id: String,
    amount: f64,
}

impl RefundRequest {
    /// Constructs a new refund request.
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

/// Represents the response to a refund operation.
#[derive(Deserialize, Debug)]
pub struct RefundResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    pub status: Status,
    /// Indicates whether the operation is a return for future payments or whether it should be performed
    /// from the merchant's account.
    pub wait_amount: Option<bool>,
    /// Represents the operation type. Possible values are `pay` - payment,
    /// `hold` - hold operation on a sender's account, `subscribe` - subscription, `paydonate` - donation.
    pub action: Option<Action>,
    /// Represents the identifier of a payment.
    pub payment_id: Option<u32>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RefundResponse {}
