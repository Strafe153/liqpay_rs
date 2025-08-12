use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Bonus, Currency, PayType, Result, Status, Version};
use crate::information::common::ResponseFormat;
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct CompensationReportRequest {
    version: Version,
    public_key: String,
    action: Action,
    #[serde(skip_serializing_if = "Option::is_none")]
    compensation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
    #[serde(rename = "resp_format")]
    response_format: ResponseFormat,
}

impl LiqPayRequest<CompensationReportResponse, Sha3_256> for CompensationReportRequest {}

impl CompensationReportRequest {
    pub fn by_compensation_id(public_key: impl Into<String>, compensation_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensation,
            public_key: public_key.into(),
            compensation_id: Some(compensation_id),
            response_format: ResponseFormat::Json,
            date: None,
        }
    }

    pub fn by_date(public_key: impl Into<String>, date: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensation,
            public_key: public_key.into(),
            date: Some(date),
            response_format: ResponseFormat::Json,
            compensation_id: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum Channel {
    #[serde(rename = "checkout")]
    Checkout,
    #[serde(rename = "checkoutjs")]
    CheckoutJs,
    #[serde(rename = "api")]
    Api,
}

// maybe some of the fields can be made just types without Option<>
#[derive(Debug, Deserialize)]
pub struct RegistryReportEntry {
    pub id: u32,
    #[serde(rename = "create_date")]
    pub creation_date: String,
    pub end_date: Option<String>,
    #[serde(rename = "trans_type")]
    pub transaction_type: Option<String>,
    #[serde(rename = "trans_amount")]
    pub transaction_amount: Option<f64>,
    #[serde(rename = "trans_fee_debit")]
    pub transaction_fee_debit: Option<f64>,
    #[serde(rename = "trans_fee_credit")]
    pub transaction_fee_credit: Option<f64>,
    #[serde(rename = "trans_bonus")]
    pub transaction_bonus: Option<f64>,
    #[serde(rename = "trans_total")]
    pub transaction_total: Option<f64>,
    #[serde(rename = "trans_currency")]
    pub transaction_currency: Option<Currency>,
    pub action: Option<Action>,
    pub channel: Option<Channel>,
    #[serde(rename = "paytype")]
    pub pay_type: Option<PayType>,
    pub order_id: Option<String>,
    pub liqpay_order_id: Option<String>,
    pub authcode_debit: Option<String>,
    pub description: Option<String>,
    pub ip: Option<String>,
    pub customer: Option<String>,
    pub bonus_type: Option<Bonus>,
    pub sender_card: Option<String>,
    pub sender_card_bank: Option<String>,
    pub sender_card_country: Option<u16>,
    pub sender_card_type: Option<String>,
    pub sender_phone: Option<String>,
    pub sender_email: Option<String>,
    pub sender_first_name: Option<String>,
    pub sender_last_name: Option<String>,
    pub sender_card_product_type: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CompensationReportResponse {
    pub result: Result,
    pub status: Option<Status>,
    pub data: Option<Vec<RegistryReportEntry>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportResponse {}

#[derive(Debug, Serialize)]
pub struct RegistryRequest {
    version: Version,
    action: Action,
    public_key: String,
    format: ResponseFormat,
    date: String,
}

impl LiqPayRequest<RegistryResponse, Sha3_256> for RegistryRequest {}

impl RegistryRequest {
    pub fn new(public_key: impl Into<String>, date: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Registry,
            public_key: public_key.into(),
            format: ResponseFormat::Json,
            date,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegistryResponse {
    pub result: Result,
    pub status: Option<Status>,
    pub data: Option<Vec<RegistryReportEntry>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RegistryResponse {}

#[derive(Debug, Serialize)]
pub struct CompensationReportFileRequest {
    version: Version,
    action: Action,
    public_key: String,
    response_format: ResponseFormat,
    #[serde(skip_serializing_if = "Option::is_none")]
    compensation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date: Option<String>,
}

impl LiqPayRequest<CompensationReportFileResposne, Sha3_256> for CompensationReportFileRequest {}

impl CompensationReportFileRequest {
    pub fn by_date(public_key: impl Into<String>, date: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFile,
            public_key: public_key.into(),
            response_format: ResponseFormat::Csv,
            date: Some(date),
            compensation_id: None,
        }
    }

    pub fn by_compensation_id(public_key: impl Into<String>, compensation_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFile,
            public_key: public_key.into(),
            response_format: ResponseFormat::Csv,
            compensation_id: Some(compensation_id),
            date: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompensationReportFileResposne {
    pub result: Result,
    pub status: Option<Status>,
    #[serde(rename = "register_token")]
    pub registration_token: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportFileResposne {}

#[derive(Debug, Serialize)]
pub struct CompensationReportFileStatusRequest {
    version: Version,
    action: Action,
    public_key: String,
    #[serde(rename = "register_token")]
    registration_token: String,
}

impl LiqPayRequest<CompensationReportFileStatusResponse, Sha3_256>
    for CompensationReportFileStatusRequest
{
}

impl CompensationReportFileStatusRequest {
    pub fn new(public_key: impl Into<String>, registration_token: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFileStatus,
            public_key: public_key.into(),
            registration_token,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CompensationReportFileStatusResponse {
    pub result: Result,
    pub status: Option<Status>,
    #[serde(rename = "filelink")]
    pub file_link: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportFileStatusResponse {}

#[derive(Debug, Serialize)]
pub enum P2PCompensationReportFileType {
    #[serde(rename = "p2p")]
    P2P,
    #[serde(rename = "p2pcredit")]
    P2PCredit,
}

#[derive(Debug, Serialize)]
pub struct P2PCompensationReportFileRequest {
    version: Version,
    action: Action,
    public_key: String,
    response_format: ResponseFormat,
    #[serde(rename = "type")]
    operation_type: P2PCompensationReportFileType,
    date: String,
}

impl LiqPayRequest<CompensationReportFileResposne, Sha3_256> for P2PCompensationReportFileRequest {}

impl P2PCompensationReportFileRequest {
    pub fn p2p(public_key: impl Into<String>, date: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFile,
            public_key: public_key.into(),
            response_format: ResponseFormat::Csv,
            operation_type: P2PCompensationReportFileType::P2P,
            date,
        }
    }

    pub fn p2p_credit(public_key: impl Into<String>, date: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFile,
            public_key: public_key.into(),
            response_format: ResponseFormat::Csv,
            operation_type: P2PCompensationReportFileType::P2PCredit,
            date,
        }
    }
}
