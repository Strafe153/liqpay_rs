use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Bonus, Currency, PayType, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::informational::common::ResponseFormat;

/// Represents a request to get a compensation report.
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
    /// Constructs a new request to get a compensation report by a compensation identifier.
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

    /// Constructs a new request to get a compensation report by a specific date.
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

/// Represents a payment channel.
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
/// Represents an entry in a registry report.
#[derive(Debug, Deserialize)]
pub struct RegistryReportEntry {
    /// Represents an identifier of a registry report entry.
    pub id: u32,
    /// Represents the payment creation date.
    #[serde(rename = "create_date")]
    pub creation_date: String,
    /// Represents the end date of the payment.
    pub end_date: Option<String>,
    /// Represents the transaction type.
    #[serde(rename = "trans_type")]
    pub transaction_type: Option<String>,
    /// Represents the transaction amount.
    #[serde(rename = "trans_amount")]
    pub transaction_amount: Option<f64>,
    /// Represents the transaction fee for debit.
    #[serde(rename = "trans_fee_debit")]
    pub transaction_fee_debit: Option<f64>,
    /// Represents the transaction fee for credit.
    #[serde(rename = "trans_fee_credit")]
    pub transaction_fee_credit: Option<f64>,
    /// Represents the transaction bonus.
    #[serde(rename = "trans_bonus")]
    pub transaction_bonus: Option<f64>,
    /// Represents the transaction total amount.
    #[serde(rename = "trans_total")]
    pub transaction_total: Option<f64>,
    /// Represents the transaction currency.
    #[serde(rename = "trans_currency")]
    pub transaction_currency: Option<Currency>,
    /// Represents the operation type. Possible values are `pay` - payment,
    /// `hold` - hold operation on a sender's account, `subscribe` - subscription, `paydonate` - donation,
    /// `auth` - card preauthentication, `regular` - regular payment, `paysplit` - split payment.
    pub action: Option<Action>,
    /// Represents a payment channel used. Possible values are `checkout`, `checkoutjs` and `api`.
    pub channel: Option<Channel>,
    /// Represents the payment type. Possible values are `card` - paid by card,
    /// `privat24` - account in the Privat24 system, `moment_part` - installment,
    /// `cash` - paid in cash, `invoice` - invoiced to an email,
    /// `qr` - paid by scanning a QR code.
    #[serde(rename = "paytype")]
    pub pay_type: Option<PayType>,
    /// Represents the identifier of an order.
    pub order_id: Option<String>,
    /// Represents the Id of an order in the LiqPay system.
    pub liqpay_order_id: Option<String>,
    /// Represents the authorization code for debit.
    pub authcode_debit: Option<String>,
    /// Represents the payment description.
    pub description: Option<String>,
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
    /// Represents an identifier of a customer.
    pub customer: Option<String>,
    /// Represents the bonus type. Possible values are `bonusplus`, `personal`, `promo` and `discount_club`.
    pub bonus_type: Option<Bonus>,
    /// Represents the sender's bonus amount.
    pub sender_bonus: Option<f64>,
    /// Represents the sender's card.
    pub sender_card: Option<String>,
    /// Represents the sender's bank.
    pub sender_card_bank: Option<String>,
    /// Represents the sender's card country code in the ISO-3166 format.
    pub sender_card_country: Option<u16>,
    /// Represents the sender's card type - either VISA or MasterCard.
    pub sender_card_type: Option<String>,
    /// Represents the sender's email address.
    pub sender_email: Option<String>,
    /// Represents the sender's first name.
    pub sender_first_name: Option<String>,
    /// Represents the sender's last name.
    pub sender_last_name: Option<String>,
    /// Represents the sender's phone number.
    pub sender_phone: Option<String>,
    /// Represents the sender's card product type.
    pub sender_card_product_type: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

/// Represents the response to a compensation report operation.
#[derive(Debug, Deserialize)]
pub struct CompensationReportResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Represents the collection of report entries.
    pub data: Option<Vec<RegistryReportEntry>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportResponse {}

/// Represents a request to get a registry report.
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
    /// Constructs a new request to get a registry report.
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

/// Represents the response to a registry report operation.
#[derive(Debug, Deserialize)]
pub struct RegistryResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Represents the collection of report entries.
    pub data: Option<Vec<RegistryReportEntry>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RegistryResponse {}

/// Represents a request to get a compensation report file.
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

impl LiqPayRequest<CompensationReportFileResponse, Sha3_256> for CompensationReportFileRequest {}

impl CompensationReportFileRequest {
    /// Constructs a new request to get a compensation report file by a compensation identifier.
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

    /// Constructs a new request to get a compensation report file by a specific date.
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

/// Represents a request to get a compensation report file.
#[derive(Debug, Deserialize)]
pub struct CompensationReportFileResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Represents a registration token to get a file.
    #[serde(rename = "register_token")]
    pub registration_token: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportFileResponse {}

/// Represents a request to get a compensation report file status.
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
    /// Constructs a new request to get a compensation report file status.
    pub fn new(public_key: impl Into<String>, registration_token: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFileStatus,
            public_key: public_key.into(),
            registration_token,
        }
    }
}

/// Represents a request to get a compensation report file status.
#[derive(Debug, Deserialize)]
pub struct CompensationReportFileStatusResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Represents a link used to get a report file.
    #[serde(rename = "filelink")]
    pub file_link: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CompensationReportFileStatusResponse {}

/// Represents a P2P compensation report file type.
#[derive(Debug, Serialize)]
pub enum P2PCompensationReportFileType {
    #[serde(rename = "p2p")]
    P2P,
    #[serde(rename = "p2pcredit")]
    P2PCredit,
}

/// Represents a request to get a P2P compensation report file.
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

impl LiqPayRequest<CompensationReportFileResponse, Sha3_256> for P2PCompensationReportFileRequest {}

impl P2PCompensationReportFileRequest {
    /// Constructs a new request to get a P2P compensation report file.
    pub fn p2p(public_key: impl Into<String>, date: String) -> Self {
        Self::new(public_key, date, P2PCompensationReportFileType::P2P)
    }

    /// Constructs a new request to get a P2P credit compensation report file.
    pub fn p2p_credit(public_key: impl Into<String>, date: String) -> Self {
        Self::new(public_key, date, P2PCompensationReportFileType::P2PCredit)
    }

    fn new(
        public_key: impl Into<String>,
        date: String,
        operation_type: P2PCompensationReportFileType,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::ReportsCompensationFile,
            public_key: public_key.into(),
            response_format: ResponseFormat::Csv,
            operation_type,
            date,
        }
    }
}
