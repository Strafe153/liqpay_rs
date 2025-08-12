use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Currency, Language, MpiEci, PayType, Status, Version};
use crate::information::common::{ResponseFormat, Result};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct ArchiveRequest {
    version: Version,
    public_key: String,
    action: Action,
    date_from: String,
    date_to: String,
    #[serde(rename = "resp_format")]
    response_format: ResponseFormat,
}

impl LiqPayRequest<ArchiveResponse, Sha3_256> for ArchiveRequest {}

impl ArchiveRequest {
    pub fn new(public_key: impl Into<String>, date_from: String, date_to: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Reports,
            public_key: public_key.into(),
            response_format: ResponseFormat::Json,
            date_from,
            date_to,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ArchiveEntry {
    pub status: Status,
    pub acq_id: Option<u32>,
    pub action: Option<Action>,
    pub agent_commission: Option<f64>,
    pub amount: Option<f64>,
    pub amount_bonus: Option<f64>,
    pub amount_credit: Option<f64>,
    pub amount_debit: Option<f64>,
    pub commission_credit: Option<f64>,
    pub commission_debit: Option<f64>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    pub currency: Option<String>,
    pub currency_credit: Option<Currency>,
    pub currency_debit: Option<Currency>,
    pub description: Option<String>,
    pub end_date: Option<u64>,
    pub is_3ds: Option<bool>,
    pub language: Option<Language>,
    pub liqpay_order_id: Option<String>,
    pub mpi_eci: Option<MpiEci>,
    pub order_id: Option<String>,
    pub payment_id: Option<u64>,
    #[serde(rename = "paytype")]
    pub pay_type: Option<PayType>,
    pub public_key: Option<String>,
    pub receiver_commission: Option<f32>,
    pub sender_bonus: Option<f64>,
    pub sender_card_bank: Option<String>,
    pub sender_card_country: Option<u16>, // ideally should be SenderCountryCode
    pub sender_card_mask2: Option<String>,
    pub sender_card_type: Option<String>,
    pub sender_commission: Option<f64>,
    pub transaction_id: Option<u64>,
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    // potentially would like to use Version enum here but don't know how to implement it yet
    pub version: Option<u8>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArchiveResponse {
    pub result: Result,
    pub status: Option<Status>,
    pub data: Option<Vec<ArchiveEntry>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for ArchiveResponse {}
