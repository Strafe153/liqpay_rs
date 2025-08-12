use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Currency, Language, MpiEci, Result, Status, Version};
use crate::internet_acquiring::common::DetailAddenda;
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Serialize, Debug)]
pub struct DynamicQrCodeRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    description: String,
    order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(rename = "dae", skip_serializing_if = "Option::is_none")]
    detail_addenda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_url: Option<String>,
}

impl LiqPayRequest<DynamicQrCodeResponse, Sha3_256> for DynamicQrCodeRequest {}

impl DynamicQrCodeRequest {
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::PayQrCode,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            ip: None,
            language: None,
            prepare: None,
            server_url: None,
            split_rules: None,
            split_tickets_only: None,
            customer: None,
            detail_addenda: None,
            info: None,
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
            recurring_by_token: None,
        }
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn prepare(mut self) -> Self {
        self.prepare = Some('1');
        self
    }

    pub fn recurring_by_token(mut self) -> Self {
        self.recurring_by_token = Some('1');
        self
    }

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn split_rules(mut self, rules: String) -> Self {
        self.split_rules = Some(rules);
        self
    }

    pub fn split_tickets_only(mut self) -> Self {
        self.split_tickets_only = Some(true);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }

    pub fn detail_addenda(mut self, detail: DetailAddenda) -> Self {
        self.detail_addenda = Some(detail.to_base64());
        self
    }

    pub fn info(mut self, info: String) -> Self {
        self.info = Some(info);
        self
    }

    pub fn product_category(mut self, category: String) -> Self {
        self.product_category = Some(category);
        self
    }

    pub fn product_description(mut self, description: String) -> Self {
        self.product_description = Some(description);
        self
    }

    pub fn product_name(mut self, name: String) -> Self {
        self.product_name = Some(name);
        self
    }

    pub fn product_url(mut self, url: String) -> Self {
        self.product_url = Some(url);
        self
    }
}

#[derive(Deserialize, Debug)]
pub struct DynamicQrCodeResponse {
    pub result: Result,
    pub status: Status,
    pub acq_id: Option<u32>,
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
    pub currency: Option<String>,
    pub currency_credit: Option<Currency>,
    pub currency_debit: Option<Currency>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    pub description: Option<String>,
    pub end_date: Option<u64>,
    pub ip: Option<String>,
    pub is_3ds: Option<bool>,
    pub liqpay_order_id: Option<String>,
    pub mpi_eci: Option<MpiEci>,
    pub order_id: Option<String>,
    pub payment_id: Option<u64>,
    pub public_key: Option<String>,
    pub qr_code: Option<String>,
    pub receiver_commission: Option<f32>,
    pub sender_bonus: Option<f64>,
    pub sender_commission: Option<f64>,
    pub transaction_id: Option<u64>,
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    pub version: Option<u8>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for DynamicQrCodeResponse {}

#[derive(Debug, Serialize)]
pub struct StaticQrCodeRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    server_url: Option<String>,
    final_date: Option<String>,
}

impl LiqPayRequest<StaticQrCodeResponse, Sha3_256> for StaticQrCodeRequest {}

impl StaticQrCodeRequest {
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CreateQrCode,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            server_url: None,
            final_date: None,
        }
    }

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn final_date(mut self, date: String) -> Self {
        self.final_date = Some(date);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct StaticQrCodeResponse {
    pub result: Result,
    pub status: Status,
    pub id: Option<u32>,
    pub shop_id: Option<u32>,
    pub amount: Option<f64>,
    pub currency: Option<Currency>,
    pub description: Option<String>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    pub final_date: Option<u64>,
    #[serde(rename = "qrdata")]
    pub qr_code_data: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for StaticQrCodeResponse {}
