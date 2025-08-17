use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha1::Sha1;

use crate::common::enums::{
    Action, Bonus, Currency, Language, MpiEci, PayType, Result, Status, Version,
};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::DetailAddenda;

/// Represents a request to perform a token-based payment.
#[derive(Serialize, Debug)]
pub struct TokenPaymentRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    card_token: String,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_postal_code: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    is_recurring: Option<bool>,
}

impl LiqPayRequest<TokenPaymentResponse, Sha1> for TokenPaymentRequest {}

impl TokenPaymentRequest {
    /// Constructs a new token payment request.
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        card_token: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Three,
            action: Action::Pay,
            public_key: public_key.into(),
            amount,
            card_token,
            currency,
            order_id,
            description,
            ip: None,
            phone: None,
            language: None,
            prepare: None,
            server_url: None,
            split_rules: None,
            split_tickets_only: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_postal_code: None,
            customer: None,
            detail_addenda: None,
            info: None,
            product_category: None,
            product_description: None,
            product_name: None,
            product_url: None,
            is_recurring: None,
        }
    }

    /// Sets the customer's IP address.
    pub fn ip(mut self, ip: String) -> Self {
        self.ip = Some(ip);
        self
    }

    /// Sets the customer's phone number, to which a one-time password will be sent for confirmation.
    /// The number should adhere to the internation format.
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
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

    /// Sets the API URL for payment status notifications.
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

    /// Sets the sender's address.
    pub fn sender_address(mut self, address: String) -> Self {
        self.sender_address = Some(address);
        self
    }

    /// Sets the sender's city.
    pub fn sender_city(mut self, city: String) -> Self {
        self.sender_city = Some(city);
        self
    }

    /// Sets the sender's country code in the ISO3166 format.
    pub fn sender_country_code(mut self, country: Country) -> Self {
        self.sender_country_code = Some(country.id.to_string());
        self
    }

    /// Sets the sender's first name.
    pub fn sender_first_name(mut self, name: String) -> Self {
        self.sender_first_name = Some(name);
        self
    }

    /// Sets the sender's last name.
    pub fn sender_last_name(mut self, name: String) -> Self {
        self.sender_last_name = Some(name);
        self
    }

    /// Sets the sender's postal code.
    pub fn sender_postal_code(mut self, code: String) -> Self {
        self.sender_postal_code = Some(code);
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

    /// Sets the payment as initiated by the merchant.
    pub fn merchant_initiated(mut self) -> Self {
        self.is_recurring = Some(true);
        self
    }

    /// Sets the payment as initiated by the customer.
    pub fn customer_initiated(mut self) -> Self {
        self.is_recurring = Some(false);
        self
    }
}

/// Represents the response to a token payment operation.
#[derive(Deserialize)]
pub struct TokenPaymentResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `wait_accept` - the store is not verified at this point, but the money are withdrawn from a client,
    /// `wait_secure` - verified payment.
    pub status: Status,
    /// Represents the identifier of an acquirer.
    #[serde(rename = "acq_id")]
    pub acquirer_id: Option<u32>,
    /// Represents the operation type. Possible values are `pay` - payment,
    /// `hold` - hold operation on a sender's account, `subscribe` - subscription, `paydonate` - donation,
    /// `auth` - card preauthentication, `regular` - regular payment.
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

impl LiqPayResponse for TokenPaymentResponse {}
