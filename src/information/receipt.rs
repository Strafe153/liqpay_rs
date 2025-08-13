use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct SendReceiptRequest {
    version: Version,
    public_key: String,
    action: Action,
    email: String,
    order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
}

impl LiqPayRequest<SendReceiptResponse, Sha3_256> for SendReceiptRequest {}

impl SendReceiptRequest {
    pub fn new(public_key: impl Into<String>, email: String, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Ticket,
            public_key: public_key.into(),
            email,
            order_id,
            payment_id: None,
            language: None,
        }
    }

    pub fn payment_id(mut self, id: String) -> Self {
        self.payment_id = Some(id);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct SendReceiptResponse {
    pub result: Result,
    pub status: Option<Status>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for SendReceiptResponse {}
