use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, Prepare, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct P2PDebitRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<Prepare>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<char>,
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
    mpi_eci: Option<MpiEci>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpi_cres: Option<String>,
}

impl LiqPayRequest<P2PDebitResponse, Sha3_256> for P2PDebitRequest {}

impl P2PDebitRequest {
    pub fn by_card(
        public_key: impl Into<String>,
        amount: f64,
        card: String,
        cvv: String,
        exp_month: String,
        exp_year: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);

        request.card = Some(card);
        request.card_cvv = Some(cvv);
        request.card_exp_month = Some(exp_month);
        request.card_exp_year = Some(exp_year);

        request
    }

    pub fn by_token(
        public_key: impl Into<String>,
        amount: f64,
        card_token: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        request.card_token = Some(card_token);

        request
    }

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn langugae(mut self, language: Language) -> Self {
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

    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    pub fn sandbox(mut self) -> Self {
        self.sandbox = Some('1');
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

    pub fn mpi_eci(mut self, eci: MpiEci) -> Self {
        self.mpi_eci = Some(eci);
        self
    }

    pub fn mpi_cres(mut self, cres: String) -> Self {
        self.mpi_cres = Some(cres);
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
            action: Action::P2PDebit,
            public_key: public_key.into(),
            amount,
            card: None,
            card_cvv: None,
            card_exp_month: None,
            card_exp_year: None,
            card_token: None,
            currency,
            order_id,
            description,
            phone: None,
            language: None,
            prepare: None,
            recurring_by_token: None,
            result_url: None,
            server_url: None,
            sandbox: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_email: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_state: None,
            sender_shipping_state: None,
            sender_postal_code: None,
            mpi_eci: None,
            mpi_cres: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct P2PDebitResponse {
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

impl LiqPayResponse for P2PDebitResponse {}
