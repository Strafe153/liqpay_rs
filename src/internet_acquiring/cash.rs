use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, PayType, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::DetailAddenda;

/// Represents a request to perform a cash payment.
#[derive(Serialize, Debug)]
pub struct CashPaymentRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(rename = "expired_date", skip_serializing_if = "Option::is_none")]
    expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(rename = "dae", skip_serializing_if = "Option::is_none")]
    detail_addenda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_url: Option<String>,
}

impl LiqPayRequest<CashPaymentResponse, Sha3_256> for CashPaymentRequest {}

impl CashPaymentRequest {
    /// Constructs a new cash payment request.
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::PayCash,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            ip: None,
            phone: None,
            expiration_date: None,
            language: None,
            prepare: None,
            server_url: None,
            split_rules: None,
            customer: None,
            detail_addenda: None,
            info: None,
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
        }
    }

    /// Sets the customer's IP address.
    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    /// Sets the customer's phone number.
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Sets the date and time (in UTC) until which a customer is able to pay an invoice.
    pub fn expiration_date(mut self, date: String) -> Self {
        self.expiration_date = Some(date);
        self
    }

    /// Sets the customer's language. Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Sets the preliminary preparation of the payment. The mode allows to determine data completeness,
    /// whether 3DS is required or limit is exceeded. The funds are not debited.
    pub fn prepare(mut self) -> Self {
        self.prepare = Some('1');
        self
    }

    /// Sets the API URL, where a notification is sent to on a payment status change.
    /// Must not exceed 510 characters.
    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    /// Sets the payment's amount to be split among several receivers. The parameter is a JSON array.
    /// The fee is charged from every specified receiver.
    pub fn split_rules(mut self, rules: String) -> Self {
        self.split_rules = Some(rules);
        self
    }

    /// Sets the unique identifier of a customer. Must not exceed 100 characters.
    pub fn customer(mut self, customer: String) -> Self {
        self.customer = Some(customer);
        self
    }

    /// Sets the transportation details.
    pub fn detail_addenda(mut self, detail: DetailAddenda) -> Self {
        self.detail_addenda = Some(detail.to_base64());
        self
    }

    /// Sets the additional information about the payment.
    pub fn info(mut self, info: String) -> Self {
        self.info = Some(info);
        self
    }

    /// Sets the product category. Must not exceed 25 symbols.
    pub fn product_category(mut self, category: String) -> Self {
        self.product_category = Some(category);
        self
    }

    /// Sets the product description. Must not exceed 500 characters.
    pub fn product_description(mut self, description: String) -> Self {
        self.product_description = Some(description);
        self
    }

    /// Sets the product name. Must not exceed 100 characters.
    pub fn product_name(mut self, name: String) -> Self {
        self.product_name = Some(name);
        self
    }

    /// Sets the product page URL. Must not exceed 2000 characters.
    pub fn product_url(mut self, url: String) -> Self {
        self.product_url = Some(url);
        self
    }
}

/// Represents the response to a cash payment operation.
#[derive(Deserialize, Debug)]
pub struct CashPaymentResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `cash_wait` - a self-service terminal is waiting for a payment.
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
    /// Represents the agent commission.
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
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
    /// Indicates whether a transaction passed with 3DS.
    pub is_3ds: Option<bool>,
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

impl LiqPayResponse for CashPaymentResponse {}
