use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to get a payment status.
#[derive(Debug, Serialize)]
pub struct StatusRequest {
    version: Version,
    public_key: String,
    action: Action,
    order_id: String,
}

impl LiqPayRequest<StatusResponse, Sha3_256> for StatusRequest {}

impl StatusRequest {
    /// Constructs a new request to get a payment status.
    pub fn new(public_key: impl Into<String>, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Status,
            public_key: public_key.into(),
            order_id,
        }
    }
}

/// Represents the response to getting a payment status operation.
#[derive(Debug, Deserialize)]
pub struct StatusResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `subscribed` - successful subscription,
    /// `success` - successful payment, `unsubscribed` - successful subscription deactivation,
    /// `3ds_verify` - the verification by 3DS is required, `captcha_verify` - captcha verification required,
    /// `cvv_verify` - CVV required, `ivr_verify` - IVR verification required, `otp_verify` - confirmation
    /// by OTP is required, `password_verify` - Privat24 application verification required,
    /// `phone_verify` - phone number verification required, `pin_verify` - PIN verification required,
    /// `receiver_verify` - additional receiver information is required, `sender_verify` - additional sender
    /// information is required, `senderapp_verify` - additional Privat24 application verification required,
    /// `wait_accept` - the store is not verified at this point, but the money are withdrawn from a client,
    /// `wait_secure` - verified payment.pub status: Status, `wait_qr` - QR code verification required,
    /// `p24_verify` - payment finalization in Privat24 required, `mp_verify` - payment finalization in
    /// MasterCard required, `cash_wait` - payment finalization in a terminal required,
    /// `hold_wait` - successful amount block on an account, `invoice_wait` - successfully created invoice is
    /// waiting for a payment, `prepared` - successful payment creation is waiting for a finalization
    /// from a customer, `processing` - payment is being processed, `wait_card` - a compensation method is not set,
    /// `wait_compensation` - successful payment will be finished in a daily settlement,
    /// `wait_lc` - successful charge of a protected payment is waiting for confirmation,
    /// `wait_reserve` - funds reservation for a refund, `try_again` - unsuccessful payment.
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
    pub authcode_debit: Option<f64>,
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
    /// Represents the phone number used for confirmation via a one-time password.
    pub confirm_phone: Option<String>,
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
    /// Represents additional information.
    pub info: Option<String>,
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
    /// Indicates whether a transaction passed with 3DS.
    pub is_3ds: Option<bool>,
    /// Represents the language of the payment.
    pub language: Option<Language>,
    /// Represents the Id of an order in the LiqPay system.
    pub liqpay_order_id: Option<String>,
    /// Represents indication of a payment in parts.
    pub moment_part: Option<String>,
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
    /// Represents an additional status of a payment indicating whether a payment is reserved for further processing a return.
    pub wait_reserve_status: Option<String>,
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

impl LiqPayResponse for StatusResponse {}
