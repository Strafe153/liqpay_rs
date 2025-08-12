use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct PartnerInformationRequest {
    version: Version,
    action: Action,
    public_key: String,
    phone: String,
}

impl LiqPayRequest<PartnerInformationResponse, Sha3_256> for PartnerInformationRequest {}

impl PartnerInformationRequest {
    pub fn new(public_key: impl Into<String>, phone: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::UserInfo,
            public_key: public_key.into(),
            phone,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PartnerInformationResponse {
    pub result: Result,
    pub status: Option<Status>,
    pub blocked: Option<String>,
    #[serde(rename = "create_date")]
    pub creation_date: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub logo: Option<String>,
    pub name: Option<String>,
    pub public_key: Option<String>,
    pub refund_number: Option<String>,
    pub refund_way: Option<String>,
    pub role: Option<String>,
    pub public_phone: Option<String>,
    pub update_date: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for PartnerInformationResponse {}
