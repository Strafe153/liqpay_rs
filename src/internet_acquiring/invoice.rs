use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::RroInfo;

/// Represents a request to initiate an invoice sending operation.
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
    /// Constructs a new invoice sending request.
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

    /// Sets a description.
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets a phone number.
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Sets fiscalization data.
    pub fn rro_info(mut self, info: RroInfo) -> Self {
        self.rro_info = Some(info);
        self
    }

    /// Sets the transaction's action type to `pay`.
    pub fn pay_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Pay);
        self
    }

    /// Sets the transaction's action type to `hold`.
    pub fn hold_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Hold);
        self
    }

    /// Sets the transaction's action type to `subscribe`.
    pub fn subscribe_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::Subscribe);
        self
    }

    /// Sets the transaction's action type to `paydonate`.
    pub fn pay_donate_action_payment(mut self) -> Self {
        self.action_payment = Some(Action::PayDonate);
        self
    }

    /// Sets the date and time (in UTC) until which a customer is able to pay an invoice.
    pub fn expiration_date(mut self, date: String) -> Self {
        self.expiration_date = Some(date);
        self
    }

    /// Sets the goods.
    pub fn goods(mut self, goods: String) -> Self {
        self.goods = Some(goods);
        self
    }

    /// Sets the customer's language. Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Sets the URL of the shop, to which the customer is redirected after completing a purchase.
    /// Must not exceed 510 characters.
    pub fn result_url(mut self, url: String) -> Self {
        self.result_url = Some(url);
        self
    }

    /// Sets the API URL, where a notification is sent to on a payment status change.
    /// Must not exceed 510 characters.
    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }
}

/// Represents the response to an invoice sending operation.
#[derive(Debug, Deserialize)]
pub struct SendInvoiceResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `invoice_wait` - a successfully created invoice is waiting for a payment.
    pub status: Status,
    /// Represents the operation type. Possible values are `pay` - payment,
    /// `hold` - hold operation on a sender's account, `subscribe` - subscription, `paydonate` - donation,
    /// `auth` - card preauthentication, `regular` - regular payment.
    pub action: Option<Action>,
    /// Represents the payment amount.
    pub amount: Option<f64>,
    /// Represents the payment currency.
    pub currency: Option<String>,
    /// Represents the payment description.
    pub description: Option<String>,
    /// Represents the invoice link.
    pub href: Option<String>,
    /// Represent the identifier of the payment.
    pub id: Option<u64>,
    /// Represents the identifier of an order.
    pub order_id: Option<String>,
    /// Represents the channel type of a receiver.
    pub receiver_type: Option<String>,
    /// Represents the value from the `receiver_type` parameter.
    pub receiver_value: Option<String>,
    /// Represents the payment token.
    pub token: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for SendInvoiceResponse {}

/// Represents a request to cancel an invoice sending operation.
#[derive(Debug, Serialize)]
pub struct CancelInvoiceRequest {
    version: Version,
    public_key: String,
    action: Action,
    order_id: String,
}

impl LiqPayRequest<CancelInvoiceResponse, Sha3_256> for CancelInvoiceRequest {}

impl CancelInvoiceRequest {
    /// Constructs a new invoice cancellation request.
    pub fn new(public_key: impl Into<String>, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CancelInvoice,
            public_key: public_key.into(),
            order_id,
        }
    }
}

/// Represents the response to an invoice cancellation operation.
#[derive(Debug, Deserialize)]
pub struct CancelInvoiceResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents inv the identifier of anoice.
    pub invoice_id: Option<u32>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CancelInvoiceResponse {}

/// Represents a request to get invoice units.
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
    /// Constructs a new request to get invoice units by language.
    /// The result contains only items in the specified locale.
    pub fn by_language(public_key: impl Into<String>, language: Language) -> Self {
        Self {
            version: Version::Three,
            action: Action::GetInvoiceUnitsByLanguage,
            public_key: public_key.into(),
            language: Some(language),
            hide_language_name: None,
        }
    }

    /// Constructs a new request to get full invoice units in both the Ukrainian and English locales.
    pub fn full(public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Three,
            action: Action::GetInvoiceUnits,
            public_key: public_key.into(),
            hide_language_name: None,
            language: None,
        }
    }
    /// Sets the customer's language. Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Hides the language name. Takes effect only when a language is set.
    pub fn hide_name_language(mut self) -> Self {
        self.hide_language_name = Some(true);
        self
    }
}

/// Represents an invoice unit.
#[derive(Debug, Deserialize)]
pub struct Unit {
    /// Represents the identifier of a unit.
    pub id: u32,
    /// Represents the fiscalization identifier of a unit.
    pub rro_unit_id: Option<u32>,
    /// Represents the full name of a unit in English.
    pub full_name_en: Option<String>,
    /// Represents the full name of a unit in Ukrainian.
    pub full_name_uk: Option<String>,
    /// Represents the full name of a unit.
    pub full_name: Option<String>,
    /// Represents the short name of a unit.
    pub short_name: Option<String>,
    /// Represents the short name of a unit in English.
    pub short_name_en: Option<String>,
    /// Represents the short name of a unit in Ukrainian.
    pub short_name_uk: Option<String>,
}

/// Represents the response to a getting invoice units operation.
#[derive(Debug, Deserialize)]
pub struct InvoiceUnitsResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represents a collection of invoice units.
    pub units: Option<Vec<Unit>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for InvoiceUnitsResponse {}
