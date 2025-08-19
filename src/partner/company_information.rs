use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to get a company's information.
#[derive(Debug, Serialize)]
pub struct CompanyInformationRequest {
    version: Version,
    action: Action,
    public_key: String,
    merchant_public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
}

impl LiqPayRequest<CompanyInformationResponse, Sha3_256> for CompanyInformationRequest {}

impl CompanyInformationRequest {
    /// Construct a new request to get a company's information.
    pub fn new(public_key: impl Into<String>, merchant_public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Seven,
            action: Action::MerchantInfo,
            public_key: public_key.into(),
            merchant_public_key: merchant_public_key.into(),
            language: None,
        }
    }

    /// Sets the customer's language. Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

/// Represents a response to getting a company's information operation.
#[derive(Debug, Deserialize)]
pub struct CompanyInformationResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Indicates whether a company is blocked.
    pub blocked: Option<String>,
    /// Represents the category of a merchant activity.
    pub category: Option<String>,
    /// Represents the full name of a company.
    pub company_name: Option<String>,
    /// Represents the company creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<String>,
    /// Represents the company description.
    pub description: Option<String>,
    /// Represents the email of a company.
    pub email: Option<String>,
    /// Represents the logo of a company.
    pub logo: Option<String>,
    /// Represents the category code of a merchant's activity type.
    #[serde(rename = "mcc")]
    pub merchant_category_code: Option<String>,
    /// Represents the MFO of a company.
    pub mfo: Option<String>,
    /// Represents the name of a company.
    pub name: Option<String>,
    /// Represents the OKPO of a company.
    pub okpo: Option<String>,
    /// Represents the phone number of a company.
    pub phone: Option<String>,
    /// Represents the public key of the shop.
    pub public_key: Option<String>,
    /// Represents the public phone number of a company.
    pub public_phone: Option<String>,
    /// Represents the refund number of a company.
    pub refund_number: Option<String>,
    /// Represents the refund way of a company.
    pub refund_way: Option<String>,
    /// Represents the error description depending on the `language` value.
    pub status_description: Option<String>,
    /// Represents the MEO operator comment.
    pub comment: Option<Vec<String>>,
    /// Represents the merchant business cabinet URL.
    pub link: Option<String>,
    /// Represents the update date of a company information.
    pub update_date: Option<String>,
    /// Represents the company URL.
    pub url: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompanyInformationResponse {}
