use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, PayType, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct CardVerificationRequest {
    version: Version,
    action: Action,
    public_key: String,
    card: String,
    card_exp_month: String,
    card_exp_year: String,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify_code: Option<char>,
}

impl LiqPayRequest<CardVerificationResponse, Sha3_256> for CardVerificationRequest {}

impl CardVerificationRequest {
    pub fn new(
        public_key: impl Into<String>,
        card: String,
        exp_month: String,
        exp_year: String,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CardVerification,
            public_key: public_key.into(),
            card,
            card_exp_month: exp_month,
            card_exp_year: exp_year,
            order_id,
            description,
            card_cvv: None,
            ip: None,
            language: None,
            verify_code: None,
        }
    }

    pub fn cvv(mut self, cvv: String) -> Self {
        self.card_cvv = Some(cvv);
        self
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn verify_code(mut self) -> Self {
        self.verify_code = Some('Y');
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct CardVerificationResponse {
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
    pub ip: Option<String>,
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
    pub sender_card_country: Option<u16>,
    #[serde(rename = "sender_card_mask2")]
    pub sender_card_mask: Option<String>,
    pub sender_card_type: Option<String>,
    pub sender_commission: Option<f64>,
    pub sender_first_name: Option<String>,
    pub sender_last_name: Option<String>,
    pub sender_phone: Option<String>,
    pub transaction_id: Option<u64>,
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    pub version: Option<Version>,
    #[serde(rename = "verifycode")]
    pub verify_code: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CardVerificationResponse {}
