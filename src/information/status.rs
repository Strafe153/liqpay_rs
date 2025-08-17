use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct StatusRequest {
    version: Version,
    public_key: String,
    action: Action,
    order_id: String,
}

impl LiqPayRequest<StatusResponse, Sha3_256> for StatusRequest {}

impl StatusRequest {
    pub fn new(public_key: impl Into<String>, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Status,
            public_key: public_key.into(),
            order_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    pub result: Result,
    pub status: Status,
    #[serde(rename = "acq_id")]
    pub acquirer_id: Option<u32>,
    pub action: Option<Action>,
    pub agent_commission: Option<f64>,
    pub amount: Option<f64>,
    pub amount_bonus: Option<f64>,
    pub amount_credit: Option<f64>,
    pub amount_debit: Option<f64>,
    pub authcode_credit: Option<String>,
    pub authcode_debit: Option<String>,
    #[serde(rename = "bonus_procent")]
    pub bonus_percent: Option<f32>,
    pub bonus_type: Option<Bonus>,
    pub card_token: Option<String>,
    pub commission_credit: Option<f64>,
    pub commission_debit: Option<f64>,
    pub confirm_phone: Option<String>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    pub currency: Option<String>,
    pub currency_credit: Option<Currency>,
    pub currency_debit: Option<Currency>,
    pub description: Option<String>,
    pub end_date: Option<u64>,
    pub info: Option<String>,
    pub ip: Option<String>,
    pub is_3ds: Option<bool>,
    pub language: Option<Language>,
    pub liqpay_order_id: Option<String>,
    pub moment_part: Option<String>,
    pub mpi_eci: Option<MpiEci>,
    pub order_id: Option<String>,
    pub payment_id: Option<u64>,
    #[serde(rename = "paytype")]
    pub pay_type: Option<PayType>,
    pub public_key: Option<String>,
    pub receiver_commission: Option<f32>,
    pub rrn_credit: Option<String>,
    /// Represents the identifier of the transaction in a the issuer bank's system for debit.
    #[serde(rename = "rrn_debit")]
    pub retrieval_reference_number_debit: Option<String>,
    pub sender_bonus: Option<f64>,
    pub sender_card_bank: Option<String>,
    pub sender_card_country: Option<u16>,
    #[serde(rename = "sender_card_mask2")]
    pub sender_card_mask: Option<String>,
    pub sender_card_type: Option<String>,
    pub sender_commission: Option<f64>,
    pub sender_first_name: Option<String>,
    pub sender_last_name: Option<String>,
    pub sender_phone: Option<String>,
    pub wait_reserve_status: Option<String>,
    pub transaction_id: Option<u64>,
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    pub version: Option<Version>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for StatusResponse {}
