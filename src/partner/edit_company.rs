use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to edit a company's information.
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
    /// Construct a new request to edit a company's information.
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

    /// Sets the percentage of an agent fee.
    pub fn amount_percent_agent(mut self, percentage: f32) -> Self {
        self.amount_percent_agent = Some(percentage);
        self
    }

    /// Sets the static fee of an agent.
    pub fn amount_static_agent(mut self, commission: f64, currency: Currency) -> Self {
        self.amount_static_agent = Some(commission);
        self.currency_static_agent = Some(currency);

        self
    }

    /// Enables the payment history of the created company to be viewed by the owner.
    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    /// Enables the checkout page to be edited by the owner.
    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    /// Sets the URL to a company's logo.
    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Sets the public phone number of a company.
    pub fn public_phone(mut self, phone: String) -> Self {
        self.public_phone = Some(phone);
        self
    }
}

/// Represents a response to editing a company's information operation.
#[derive(Debug, Deserialize)]
pub struct EditCompanyResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represents the private key of a company.
    pub private_key: Option<String>,
    /// Represents the public key of a company.
    pub public_key: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for EditCompanyResponse {}
