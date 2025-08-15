use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::DetailAddenda;

#[derive(Serialize, Debug)]
pub struct TokenPaymentRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    card_token: String,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_postal_code: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    is_recurring: Option<bool>,
}

impl LiqPayRequest<TokenPaymentResponse, Sha1> for TokenPaymentRequest {}

impl TokenPaymentRequest {
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        card_token: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Three,
            action: Action::Pay,
            public_key: public_key.into(),
            amount,
            card_token,
            currency,
            order_id,
            description,
            ip: None,
            phone: None,
            language: None,
            prepare: None,
            server_url: None,
            split_rules: None,
            split_tickets_only: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_postal_code: None,
            customer: None,
            detail_addenda: None,
            info: None,
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
            is_recurring: None,
        }
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
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

    pub fn sender_address(mut self, address: String) -> Self {
        self.sender_address = Some(address);
        self
    }

    pub fn sender_city(mut self, city: String) -> Self {
        self.sender_city = Some(city);
        self
    }

    pub fn sender_country_code(mut self, country: Country) -> Self {
        self.sender_country_code = Some(country.id.to_string());
        self
    }

    pub fn sender_first_name(mut self, name: String) -> Self {
        self.sender_first_name = Some(name);
        self
    }

    pub fn sender_last_name(mut self, name: String) -> Self {
        self.sender_last_name = Some(name);
        self
    }

    pub fn sender_postal_code(mut self, code: String) -> Self {
        self.sender_postal_code = Some(code);
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

    pub fn recurring(mut self) -> Self {
        self.is_recurring = Some(true);
        self
    }
}

#[derive(Deserialize)]
pub struct TokenPaymentResponse {
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
    pub rrn_credit: Option<String>,
    pub rrn_debit: Option<String>,
    pub sender_bonus: Option<f64>,
    pub sender_card_bank: Option<String>,
    pub sender_card_country: Option<u16>, // ideally should be SenderCountryCode
    pub sender_card_mask2: Option<String>,
    pub sender_card_type: Option<String>,
    pub sender_commission: Option<f64>,
    pub sender_first_name: Option<String>,
    pub sender_last_name: Option<String>,
    pub sender_phone: Option<String>,
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

impl LiqPayResponse for TokenPaymentResponse {}
