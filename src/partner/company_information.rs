use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Language, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

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
    pub fn new(public_key: impl Into<String>, merchant_public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Seven,
            action: Action::MerchantInfo,
            public_key: public_key.into(),
            merchant_public_key: merchant_public_key.into(),
            language: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct CompanyInformationResponse {
    pub result: Result,
    pub status: Status,
    pub blocked: Option<String>,
    pub category: Option<String>,
    pub company_name: Option<String>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub logo: Option<String>,
    pub mcc: Option<String>,
    pub mfo: Option<String>,
    pub name: Option<String>,
    pub okpo: Option<String>,
    pub phone: Option<String>,
    pub public_key: Option<String>,
    pub public_phone: Option<String>,
    pub refund_number: Option<String>,
    pub refund_way: Option<String>,
    pub status_description: Option<String>,
    pub comment: Option<String>,
    pub link: Option<String>,
    pub update_date: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompanyInformationResponse {}
