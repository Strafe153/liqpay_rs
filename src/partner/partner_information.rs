use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to get a partner's information.
#[derive(Debug, Serialize)]
pub struct PartnerInformationRequest {
    version: Version,
    action: Action,
    public_key: String,
    phone: String,
}

impl LiqPayRequest<PartnerInformationResponse, Sha3_256> for PartnerInformationRequest {}

impl PartnerInformationRequest {
    /// Construct a new request to get a partner's information.
    pub fn new(public_key: impl Into<String>, phone: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::UserInfo,
            public_key: public_key.into(),
            phone,
        }
    }
}

/// Represents a response to to getting a partner's information operation.
#[derive(Debug, Deserialize)]
pub struct PartnerInformationResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Indicates whether a company is blocked.
    pub blocked: Option<String>,
    /// Represents the company creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<String>,
    /// Represents the company description.
    pub description: Option<String>,
    /// Represents the email of a company.
    pub email: Option<String>,
    /// Represents the logo of a company.
    pub logo: Option<String>,
    /// Represents the name of a company.
    pub name: Option<String>,
    /// Represents the public key of the shop.
    pub public_key: Option<String>,
    /// Represents the refund number of a company.
    pub refund_number: Option<String>,
    /// Represents the refund way of a company.
    pub refund_way: Option<String>,
    /// Represents the role of a user, that initiated the operation.
    pub role: Option<String>,
    /// Represents the public phone number of a company.
    pub public_phone: Option<String>,
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

impl LiqPayResponse for PartnerInformationResponse {}
