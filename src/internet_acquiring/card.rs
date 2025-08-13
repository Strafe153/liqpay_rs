use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Prepare, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::{DetailAddenda, RroInfo};

#[derive(Debug, Serialize)]
pub struct CardPaymentRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    card: String,
    card_exp_month: String,
    card_exp_year: String,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(rename = "paytype", skip_serializing_if = "Option::is_none")]
    pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<Prepare>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    eci: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cavv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tdsv: Option<String>,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    ds_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rro_info: Option<RroInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
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

impl LiqPayRequest<CardPaymentResponse, Sha3_256> for CardPaymentRequest {}

impl CardPaymentRequest {
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card: String,
        exp_month: String,
        exp_year: String,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Pay,
            public_key: public_key.into(),
            amount,
            card,
            card_exp_month: exp_month,
            card_exp_year: exp_year,
            currency,
            order_id,
            description,
            card_cvv: None,
            ip: None,
            pay_type: None,
            phone: None,
            tavv: None,
            tid: None,
            language: None,
            prepare: None,
            recurring_by_token: None,
            result_url: None,
            recurring: None,
            server_url: None,
            eci: None,
            cavv: None,
            tdsv: None,
            ds_trans_id: None,
            rro_info: None,
            split_rules: None,
            split_tickets_only: None,
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
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
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

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn apple_pay_type(self, tavv: impl Into<String>) -> Self {
        self.tavv(tavv, PayType::ApplePay)
    }

    pub fn apple_pay_type_tavv(self, tavv: impl Into<String>) -> Self {
        self.tavv(tavv, PayType::ApplePayDecrypted)
    }

    pub fn google_pay_type(self, tavv: impl Into<String>) -> Self {
        self.tavv(tavv, PayType::GooglePay)
    }

    pub fn google_pay_type_tavv(self, tavv: impl Into<String>) -> Self {
        self.tavv(tavv, PayType::ApplePayDecrypted)
    }

    pub fn tid(mut self, tid: String) -> Self {
        self.tid = Some(tid);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn prepare(mut self, prepare: Prepare) -> Self {
        self.prepare = Some(prepare);
        self
    }

    pub fn recurring_by_token(mut self) -> Self {
        self.recurring_by_token = Some('1');
        self
    }

    pub fn result_url(mut self, url: String) -> Self {
        self.result_url = Some(url);
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

    pub fn rro_info(mut self, info: RroInfo) -> Self {
        self.rro_info = Some(info);
        self
    }

    pub fn split_tickets_only(mut self) -> Self {
        self.split_tickets_only = Some(true);
        self
    }

    pub fn split_rules(mut self, rules: String) -> Self {
        self.split_rules = Some(rules);
        self
    }

    pub fn sender_first(mut self, name: String) -> Self {
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

    fn tavv(mut self, tavv: impl Into<String>, pay_type: PayType) -> Self {
        self.pay_type = Some(pay_type);
        self.tavv = Some(tavv.into());

        self
    }
}

#[derive(Debug, Deserialize)]
pub struct CardPaymentResponse {
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
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    pub currency: Option<String>,
    pub currency_credit: Option<Currency>,
    pub currency_debit: Option<Currency>,
    pub description: Option<String>,
    pub end_date: Option<u64>,
    pub ip: Option<String>,
    pub is_3ds: Option<bool>,
    pub liqpay_order_id: Option<String>,
    pub mpi_eci: Option<MpiEci>,
    pub mpi_cres: Option<String>,
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
    pub language: Option<Language>,
    pub confirm_phone: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CardPaymentResponse {}
