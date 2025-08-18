use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to confirm an operation with a one-time password.
#[derive(Debug, Serialize)]
pub struct OtpRequest {
    version: Version,
    action: Action,
    public_key: String,
    otp: String,
    #[serde(rename = "confirm_token")]
    token: String,
}

impl LiqPayRequest<OtpResponse, Sha3_256> for OtpRequest {}

impl OtpRequest {
    /// Constructs a new request to confirm an operation with a one-time password.
    pub fn new(public_key: impl Into<String>, otp: String, token: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Confirm,
            public_key: public_key.into(),
            otp,
            token,
        }
    }
}

/// Represents the response to a one-time password confirmation operation.
#[derive(Debug, Deserialize)]
pub struct OtpResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `3ds_verify` - the verification by 3DS is required, `wait_secure` - verified payment,
    /// `wait_accept` - the store is not verified at this point, but the money are withdrawn from a client.
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
    /// Represents the authorization code for credit.
    pub authcode_credit: Option<String>,
    /// Represents the authorization code for debit.
    pub authcode_debit: Option<String>,
    /// Represents the bonus percentage.
    #[serde(rename = "bonus_procent")]
    pub bonus_percent: Option<f32>,
    /// Represents the bonus type. Possible values are `bonusplus`, `personal`, `promo` and `discount_club`.
    pub bonus_type: Option<Bonus>,
    /// Represents the sender's card token.
    pub card_token: Option<String>,
    /// Represents the commission charged to credit.
    pub commission_credit: Option<f64>,
    /// Represents the commission charged to debit.
    pub commission_debit: Option<f64>,
    /// Represents the payment creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    /// Represents the phone number used for confirmation via a one-time password.
    pub confirm_phone: Option<String>,
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
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
    /// Indicates whether a transaction passed with 3DS.
    pub is_3ds: Option<bool>,
    /// Represents the language of the payment.
    pub language: Option<Language>,
    /// Represents the Id of an order in the LiqPay system.
    pub liqpay_order_id: Option<String>,
    /// Represents the MPI ECI code. Possible values are `5` - passed with 3DS,
    /// `6` - 3DS is not supported by the card's issuer, `7` - passed without 3DS
    pub mpi_eci: Option<MpiEci>,
    /// Represents ACS value after authentication, returned by the issuer.
    pub mpi_cres: Option<String>,
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
    /// Represents the identifier of the transaction in a the issuer bank's system for credit.
    #[serde(rename = "rrn_credit")]
    pub retrieval_reference_number_credit: Option<String>,
    /// Represents the identifier of the transaction in a the issuer bank's system for debit.
    #[serde(rename = "rrn_debit")]
    pub retrieval_reference_number_debit: Option<String>,
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
    /// Represents the sender's first name.
    pub sender_first_name: Option<String>,
    /// Represents the sender's last name.
    pub sender_last_name: Option<String>,
    /// Represents the sender's phone number.
    pub sender_phone: Option<String>,
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

impl LiqPayResponse for OtpResponse {}
