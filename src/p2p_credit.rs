use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Currency, Language, MpiEci, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct P2PCreditRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    taxed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_mfo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_okpo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_card_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    receiver_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
}

impl LiqPayRequest<P2PCreditResponse, Sha3_256> for P2PCreditRequest {}

impl P2PCreditRequest {
    pub fn by_card(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card: String,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        request.receiver_card = Some(card);

        request
    }

    pub fn by_card_token(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card_token: String,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        request.receiver_card_token = Some(card_token);

        request
    }

    pub fn by_account(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        account: String,
        mfo: String,
        okpo: String,
        company: String,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);

        request.receiver_account = Some(account);
        request.receiver_mfo = Some(mfo);
        request.receiver_okpo = Some(okpo);
        request.receiver_company = Some(company);

        request
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn taxed(mut self) -> Self {
        self.taxed = Some(String::from("Дохід не підлягає оподаткуванню"));
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

    pub fn sender_postal_code(mut self, code: String) -> Self {
        self.sender_postal_code = Some(code);
        self
    }

    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
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
            action: Action::P2PCredit,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            receiver_card: None,
            ip: None,
            language: None,
            server_url: None,
            taxed: None,
            receiver_account: None,
            receiver_mfo: None,
            receiver_okpo: None,
            receiver_company: None,
            receiver_card_token: None,
            receiver_first_name: None,
            receiver_last_name: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_postal_code: None,
            customer: None,
            info: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct P2PCreditResponse {
    pub result: Result,
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
    pub liqpay_order_id: Option<String>,
    pub mpi_eci: Option<MpiEci>,
    pub order_id: Option<String>,
    pub payment_id: Option<u64>,
    pub public_key: Option<String>,
    pub receiver_commission: Option<f32>,
    pub redirect_to: Option<String>,
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

impl LiqPayResponse for P2PCreditResponse {}
