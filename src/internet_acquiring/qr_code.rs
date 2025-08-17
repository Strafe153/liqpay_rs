use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::DetailAddenda;

/// Represents a request to perform a payment by a dynamic QR code.
#[derive(Serialize, Debug)]
pub struct DynamicQrCodeRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    description: String,
    order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
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

impl LiqPayRequest<DynamicQrCodeResponse, Sha3_256> for DynamicQrCodeRequest {}

impl DynamicQrCodeRequest {
    /// Constructs a new dynamic QR code payment request.
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::PayQrCode,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            ip: None,
            language: None,
            prepare: None,
            server_url: None,
            split_rules: None,
            split_tickets_only: None,
            customer: None,
            detail_addenda: None,
            info: None,
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
            recurring_by_token: None,
        }
    }

    /// Sets the customer's IP address.
    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
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

    /// Sets the payer `card_token` generation, which is then received in the `server_url`.
    pub fn recurring_by_token(mut self) -> Self {
        self.recurring_by_token = Some('1');
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

    /// Allows simulating a split without dividing the funds.
    /// If `split_rules` are provided, the payment will not be split.
    pub fn split_tickets_only(mut self) -> Self {
        self.split_tickets_only = Some(true);
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

/// Represents the response to a dynamic QR code operation.
#[derive(Deserialize, Debug)]
pub struct DynamicQrCodeResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `wait_qr` - waiting for a QR code to be scanned by a customer.
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
    /// Represents the authorization code for credit.
    pub authcode_credit: Option<String>,
    /// Represents the authorization code for debit.
    pub authcode_debit: Option<String>,
    /// Represents the commission charged to credit.
    pub commission_credit: Option<f64>,
    /// Represents the commission charged to debit.
    pub commission_debit: Option<f64>,
    /// Represents the currency of the payment.
    pub currency: Option<String>,
    /// Represents the currency used for credit.
    pub currency_credit: Option<Currency>,
    /// Represents the currency used for debit.
    pub currency_debit: Option<Currency>,
    /// Represents the payment creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
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
    /// Represents the public key of the shop.
    pub public_key: Option<String>,
    /// Represents the generated QR code.
    pub qr_code: Option<String>,
    /// Represents the receiver's commission.
    pub receiver_commission: Option<f32>,
    /// Represents the sender's bonus amount.
    pub sender_bonus: Option<f64>,
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

impl LiqPayResponse for DynamicQrCodeResponse {}

/// Represents a request to perform a payment by a static QR code.
#[derive(Debug, Serialize)]
pub struct StaticQrCodeRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    server_url: Option<String>,
    final_date: Option<String>,
}

impl LiqPayRequest<StaticQrCodeResponse, Sha3_256> for StaticQrCodeRequest {}

/// Represents the response to a static QR code operation.
impl StaticQrCodeRequest {
    /// Constructs a new static QR code payment request.
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CreateQrCode,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            server_url: None,
            final_date: None,
        }
    }

    /// Sets the API URL, where a notification is sent to on a payment status change.
    /// Must not exceed 510 characters.
    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    /// Sets the date and time (in UNIX format) until which a QR code is valid.
    pub fn final_date(mut self, date: String) -> Self {
        self.final_date = Some(date);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct StaticQrCodeResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represent the identifier of the payment.
    pub id: Option<u32>,
    /// Represent the identifier of a merchant in LiqPay.
    pub shop_id: Option<u32>,
    /// Represents the agent commission.
    /// Represents the payment amount.
    pub amount: Option<f64>,
    /// Represents the currency of the payment.
    pub currency: Option<Currency>,
    /// Represents the payment description.
    pub description: Option<String>,
    /// Represents the payment creation date.
    #[serde(rename = "create_date")]
    pub creation_date: Option<u64>,
    /// Represents the payment date, until which a QR code is valid.
    pub final_date: Option<u64>,
    /// Represents the generated QR code.
    #[serde(rename = "qrdata")]
    pub qr_code: Option<String>,
    /// Represents the payment URL.
    pub url: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for StaticQrCodeResponse {}
