use base64::{Engine, engine::general_purpose};
use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Currency, Language, MpiEci, PayType, Result, Status, Version};
use crate::internet_acquiring::common::DetailAddenda;
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DigitalWallet {
    ApplePay,
    GooglePay,
}

#[derive(Serialize, Debug)]
pub struct TwoStageRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(rename = "applepay_token", skip_serializing_if = "Option::is_none")]
    apple_pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_year: Option<String>,
    #[serde(rename = "gpay_token", skip_serializing_if = "Option::is_none")]
    google_pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(rename = "paytype", skip_serializing_if = "Option::is_none")]
    pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tdsv: Option<String>,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    ds_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_shipping_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_addenda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
}

impl LiqPayRequest<TwoStageResponse, Sha3_256> for TwoStageRequest {}

impl TwoStageRequest {
    pub fn card(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card: String,
        exp_month: String,
        exp_year: String,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);

        request.card = Some(card);
        request.card_exp_month = Some(exp_month);
        request.card_exp_year = Some(exp_year);
        
        request
    }

    pub fn digital_wallet(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        token: impl Into<String>,
        digital_wallet: DigitalWallet,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        let base64_token = general_purpose::STANDARD.encode(token.into());

        match digital_wallet {
            DigitalWallet::ApplePay => {
                request.apple_pay_token = Some(base64_token);
                request.pay_type = Some(PayType::ApplePay);
            }
            DigitalWallet::GooglePay => {
                request.google_pay_token = Some(base64_token);
                request.pay_type = Some(PayType::GooglePay);
            }
        };

        request
    }

    pub fn tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::Tavv);
        self.tavv = Some(tavv.into());

        self
    }

    pub fn apple_pay_tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::ApplePayDecrypted);
        self.tavv = Some(tavv.into());

        self
    }

    pub fn google_pay_tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::GooglePayDecrypted);
        self.tavv = Some(tavv.into());

        self
    }

    pub fn cvv(mut self, cvv: String) -> Self {
        self.card_cvv = Some(cvv);
        self
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn tid(mut self, tid: String) -> Self {
        self.tid = Some(tid);
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

    pub fn recurring(mut self) -> Self {
        self.recurring = Some(true);
        self
    }

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn eci(mut self, eci: String) -> Self {
        self.eci = Some(eci);
        self
    }

    pub fn cavv(mut self, cavv: String) -> Self {
        self.cavv = Some(cavv);
        self
    }

    pub fn tdsv(mut self, tdsv: String) -> Self {
        self.tdsv = Some(tdsv);
        self
    }

    pub fn ds_trans_id(mut self, id: String) -> Self {
        self.ds_trans_id = Some(id);
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

    pub fn sender_email(mut self, email: String) -> Self {
        self.sender_email = Some(email);
        self
    }

    pub fn sender_country_code(mut self, country: Country) -> Self {
        self.sender_country_code = Some(country.id.to_string());
        self
    }

    pub fn sender_city(mut self, city: String) -> Self {
        self.sender_city = Some(city);
        self
    }

    pub fn sender_address(mut self, address: String) -> Self {
        self.sender_address = Some(address);
        self
    }

    pub fn sender_state(mut self, state: String) -> Self {
        self.sender_state = Some(state);
        self
    }

    pub fn sender_shipping_state(mut self, state: String) -> Self {
        self.sender_shipping_state = Some(state);
        self
    }

    pub fn sender_postal_code(mut self, code: String) -> Self {
        self.sender_postal_code = Some(code);
        self
    }

    pub fn split_rules(mut self, rules: String) -> Self {
        self.split_rules = Some(rules);
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

    fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Hold,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            card: None,
            card_exp_month: None,
            card_exp_year: None,
            apple_pay_token: None,
            google_pay_token: None,
            pay_type: None,
            eci: None,
            cavv: None,
            tdsv: None,
            ds_trans_id: None,
            card_cvv: None,
            ip: None,
            phone: None,
            tavv: None,
            tid: None,
            language: None,
            prepare: None,
            recurring_by_token: None,
            recurring: None,
            server_url: None,
            split_rules: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_email: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_state: None,
            sender_shipping_state: None,
            sender_postal_code: None,
            customer: None,
            detail_addenda: None,
            info: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TwoStageResponse {
    pub result: Result,
    pub status: Status,
    pub acq_id: Option<u32>,
    pub action: Option<Action>,
    pub agent_commission: Option<f64>,
    pub amount: Option<f64>,
    pub amount_bonus: Option<f64>,
    pub amount_credit: Option<f64>,
    pub amount_debit: Option<f64>,
    pub authcode_debit: Option<String>,
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
    pub ip: Option<String>,
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
    pub rrn_debit: Option<String>,
    pub sender_bonus: Option<f64>,
    pub sender_card_bank: Option<String>,
    pub sender_card_country: Option<u16>,
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

impl LiqPayResponse for TwoStageResponse {}
