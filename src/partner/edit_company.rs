use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Currency, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct EditCompanyRequest {
    version: Version,
    public_key: String,
    action: Action,
    description: String,
    email: String,
    merchant_public_key: String,
    name: String,
    phone: String,
    site: String,
    iban: String,
    company: String,
    okpo: String,
    #[serde(
        rename = "amount_procent_agent",
        skip_serializing_if = "Option::is_none"
    )]
    amount_percent_agent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_static_agent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_static_agent: Option<Currency>,
    #[serde(rename = "can_reports", skip_serializing_if = "Option::is_none")]
    enable_reports: Option<String>,
    #[serde(rename = "can_checkout_edit", skip_serializing_if = "Option::is_none")]
    enable_checkout_edit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_phone: Option<String>,
}

impl LiqPayRequest<EditCompanyResponse, Sha3_256> for EditCompanyRequest {}

impl EditCompanyRequest {
    pub fn new(
        public_key: impl Into<String>,
        description: String,
        email: String,
        merchant_public_key: impl Into<String>,
        name: String,
        phone: String,
        site: String,
        iban: String,
        company: String,
        okpo: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::EditShop,
            public_key: public_key.into(),
            description,
            email,
            merchant_public_key: merchant_public_key.into(),
            name,
            phone,
            site,
            iban,
            company,
            okpo,
            amount_percent_agent: None,
            amount_static_agent: None,
            currency_static_agent: None,
            enable_reports: None,
            enable_checkout_edit: None,
            logo: None,
            public_phone: None,
        }
    }

    pub fn amount_procent_agent(mut self, procent: f32) -> Self {
        self.amount_percent_agent = Some(procent);
        self
    }

    pub fn amount_static_agent(mut self, commission: f64, currency: Currency) -> Self {
        self.amount_static_agent = Some(commission);
        self.currency_static_agent = Some(currency);

        self
    }

    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    pub fn public_phone(mut self, phone: String) -> Self {
        self.public_phone = Some(phone);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct EditCompanyResponse {
    pub result: Result,
    pub status: Status,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for EditCompanyResponse {}
