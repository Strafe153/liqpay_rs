use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, PayType, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::informational::common::ResponseFormat;

/// Represents a request to get an archive of received payments.
#[derive(Debug, Serialize)]
pub struct ArchiveRequest {
    version: Version,
    public_key: String,
    action: Action,
    date_from: String,
    date_to: String,
    #[serde(rename = "resp_format")]
    response_format: ResponseFormat,
}

impl LiqPayRequest<ArchiveResponse, Sha3_256> for ArchiveRequest {}

impl ArchiveRequest {
    /// Constructs a new request to get an archive of received payments.
    pub fn new(public_key: impl Into<String>, date_from: String, date_to: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Reports,
            public_key: public_key.into(),
            response_format: ResponseFormat::Json,
            date_from,
            date_to,
        }
    }
}

/// Represents an archive payment entry.
#[derive(Debug, Deserialize)]
pub struct ArchiveEntry {
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `3ds_verify` - the verification by 3DS is required, `cvv_verify` - CVV required,
    /// `otp_verify` - confirmation by OTP is required, `receiver_verify` - additional receiver
    /// information is required, `sender_verify` - additional sender information is required,
    /// `wait_accept` - the store is not verified at this point, but the money are withdrawn from a client,
    /// `wait_secure` - verified payment.
    pub status: Status,
    /// Represents the identifier of an acquirer.
    #[serde(rename = "acq_id")]
    pub acquirer_id: Option<u32>,
    /// Represents the operation type. Possible values are `pay` - payment,
    /// `hold` - hold operation on a sender's account, `subscribe` - subscription, `paydonate` - donation,
    /// `auth` - card preauthentication, `regular` - regular payment, `paysplit` - split payment.
    pub action: Option<Action>,
    /// Represents the agent commission.
    pub agent_commission: Option<f64>,
    /// Represents the payment amount.
    pub amount: Option<f64>,
    /// Represents the payer bonus amount.
    pub amount_bonus: Option<f64>,
    /// Represents the payment credit amount.
    pub amount_credit: Option<f64>,
    /// Represents the payment debit amount.
    pub amount_debit: Option<f64>,
    /// Represents the commission charged to credit.
    pub commission_credit: Option<f64>,
    /// Represents the commission charged to debit.
    pub commission_debit: Option<f64>,
    /// Represents the payment creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    /// Represents the currency of the payment.
    pub currency: Option<String>,
    /// Represents the currency used for credit.
    pub currency_credit: Option<Currency>,
    /// Represents the currency used for debit.
    pub currency_debit: Option<Currency>,
    /// Represents the payment description.
    pub description: Option<String>,
    /// Represents the end date of the payment.
    pub end_date: Option<u64>,
    /// Indicates whether a transaction passed with 3DS.
    pub is_3ds: Option<bool>,
    /// Represents the language of the payment.
    pub language: Option<Language>,
    /// Represents the Id of an order in the LiqPay system.
    pub liqpay_order_id: Option<String>,
    /// Represents the MPI ECI code. Possible values are `5` - passed with 3DS,
    /// `6` - 3DS is not supported by the card's issuer, `7` - passed without 3DS
    pub mpi_eci: Option<MpiEci>,
    /// Represents the identifier of an order.
    pub order_id: Option<String>,
    /// Represents the identifier of a payment.
    pub payment_id: Option<u64>,
    /// Represents the payment type. Possible values are `card` - paid by card,
    /// `privat24` - account in the Privat24 system, `moment_part` - installment,
    /// `cash` - paid in cash, `invoice` - invoiced to an email,
    /// `qr` - paid by scanning a QR code.
    #[serde(rename = "paytype")]
    pub pay_type: Option<PayType>,
    /// Represents the public key of the shop.
    pub public_key: Option<String>,
    /// Represents the receiver's commission.
    pub receiver_commission: Option<f32>,
    /// Represents the sender's bonus amount.
    pub sender_bonus: Option<f64>,
    /// Represents the sender's bank.
    pub sender_card_bank: Option<String>,
    /// Represents the sender's card country code in the ISO-3166 format.
    pub sender_card_country: Option<u16>,
    /// Represents the masked sender card number.
    #[serde(rename = "sender_card_mask2")]
    pub sender_card_mask: Option<String>,
    /// Represents the sender's card type - either VISA or MasterCard.
    pub sender_card_type: Option<String>,
    /// Represents the sender's commission amount.
    pub sender_commission: Option<f64>,
    /// Represents the identifier of a transaction in LiqPay.
    pub transaction_id: Option<u64>,
    /// Represents the type of an operation.
    #[serde(rename = "type")]
    pub operation_type: Option<String>,
    /// Represents the API version.
    pub version: Option<Version>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

/// Represents an operation result.
#[derive(Debug, Deserialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

/// Represents the response to getting an archive of received payments operation.
#[derive(Debug, Deserialize)]
pub struct ArchiveResponse {
    /// Represents the result of the request. Can be either `success` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Represents the collection of archive entries.
    pub data: Option<Vec<ArchiveEntry>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for ArchiveResponse {}
