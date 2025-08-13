use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::RroInfo;

#[derive(Debug, Serialize)]
pub struct SendInvoiceRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rro_info: Option<RroInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_payment: Option<Action>,
    #[serde(rename = "expired_date", skip_serializing_if = "Option::is_none")]
    expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    goods: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
}

impl LiqPayRequest<SendInvoiceResponse, Sha3_256> for SendInvoiceRequest {}

impl SendInvoiceRequest {
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        email: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::SendInvoice,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            email,
            description: None,
            phone: None,
            rro_info: None,
            action_payment: None,
            expiration_date: None,
            goods: None,
            language: None,
            result_url: None,
            server_url: None,
        }
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    pub fn rro_info(mut self, info: RroInfo) -> Self {
        self.rro_info = Some(info);
        self
    }

    pub fn pay_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Pay);
        self
    }

    pub fn hold_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Hold);
        self
    }

    pub fn subscribe_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Subscribe);
        self
    }

    pub fn pay_donate_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::PayDonate);
        self
    }

    pub fn expiration_date(mut self, date: String) -> Self {
        self.expiration_date = Some(date);
        self
    }

    pub fn goods(mut self, goods: String) -> Self {
        self.goods = Some(goods);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
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
}

#[derive(Debug, Deserialize)]
pub struct SendInvoiceResponse {
    pub result: Result,
    pub status: Status,
    pub action: Option<Action>,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub href: Option<String>,
    pub id: Option<u64>,
    pub order_id: Option<String>,
    pub receiver_type: Option<String>,
    pub receiver_value: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for SendInvoiceResponse {}

#[derive(Debug, Serialize)]
pub struct CancelInvoiceRequest {
    version: Version,
    public_key: String,
    action: Action,
    order_id: String,
}

impl LiqPayRequest<CancelInvoiceResponse, Sha3_256> for CancelInvoiceRequest {}

impl CancelInvoiceRequest {
    pub fn new(public_key: impl Into<String>, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CancelInvoice,
            public_key: public_key.into(),
            order_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CancelInvoiceResponse {
    pub result: Result,
    pub invoice_id: Option<u32>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CancelInvoiceResponse {}

#[derive(Debug, Serialize)]
pub struct InvoiceUnitsRequest {
    version: Version,
    action: Action,
    public_key: String,
    #[serde(rename = "hide_name_lang", skip_serializing_if = "Option::is_none")]
    hide_language_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
}

impl LiqPayRequest<InvoiceUnitsResponse, Sha1> for InvoiceUnitsRequest {}

impl InvoiceUnitsRequest {
    pub fn by_language(public_key: impl Into<String>, language: Language) -> Self {
        Self {
            version: Version::Three,
            action: Action::GetInvoiceUnitsByLanguage,
            public_key: public_key.into(),
            language: Some(language),
            hide_language_name: None,
        }
    }

    pub fn full(public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Three,
            action: Action::GetInvoiceUnits,
            public_key: public_key.into(),
            hide_language_name: None,
            language: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn hide_name_language(mut self) -> Self {
        self.hide_language_name = Some(true);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct Unit {
    pub id: u32,
    pub rro_unit_id: Option<u32>,
    pub full_name_en: Option<String>,
    pub full_name_uk: Option<String>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub short_name_en: Option<String>,
    pub short_name_uk: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct InvoiceUnitsResponse {
    pub result: Result,
    pub status: Status,
    pub units: Option<Vec<Unit>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for InvoiceUnitsResponse {}
