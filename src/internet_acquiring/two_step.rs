use base64::{Engine, engine::general_purpose};
use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, PayType, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};
use crate::internet_acquiring::common::{DetailAddenda, ElectronicCommerceIndicator, RroInfo};

/// Represents a digital wallet option.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DigitalWallet {
    ApplePay,
    GooglePay,
}

/// Represents a request to block funds.
#[derive(Serialize, Debug)]
pub struct FundsBlockingRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(rename = "applepay_token", skip_serializing_if = "Option::is_none")]
    apple_pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_year: Option<String>,
    #[serde(rename = "gpay_token", skip_serializing_if = "Option::is_none")]
    google_pay_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(rename = "paytype", skip_serializing_if = "Option::is_none")]
    pay_type: Option<PayType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<char>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tavv: Option<String>,
    #[serde(rename = "eci", skip_serializing_if = "Option::is_none")]
    electronic_commerce_indicator: Option<ElectronicCommerceIndicator>,
    #[serde(rename = "cavv", skip_serializing_if = "Option::is_none")]
    cardholder_authentication_verification_value: Option<String>,
    #[serde(rename = "tdsv", skip_serializing_if = "Option::is_none")]
    three_ds_version: Option<String>,
    #[serde(rename = "dsTransID", skip_serializing_if = "Option::is_none")]
    three_ds_transaction_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_shipping_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_rules: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail_addenda: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    info: Option<String>,
}

impl LiqPayRequest<FundsBlockingResponse, Sha3_256> for FundsBlockingRequest {}

impl FundsBlockingRequest {
    /// Constructs a new funds blocking request by a payment card.
    pub fn card(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card: String,
        exp_month: String,
        exp_year: String,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);

        request.card = Some(card);
        request.card_exp_month = Some(exp_month);
        request.card_exp_year = Some(exp_year);

        request
    }

    /// Constructs a new funds blocking request by a digital wallet.
    pub fn digital_wallet(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        token: impl Into<String>,
        digital_wallet: DigitalWallet,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        let base64_token = general_purpose::STANDARD.encode(token.into());

        match digital_wallet {
            DigitalWallet::ApplePay => {
                request.apple_pay_token = Some(base64_token);
                request.pay_type = Some(PayType::ApplePay);
            }
            DigitalWallet::GooglePay => {
                request.google_pay_token = Some(base64_token);
                request.pay_type = Some(PayType::GooglePay);
            }
        };

        request
    }

    /// Sets a decrypted token from an apple device.
    pub fn tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::Tavv);
        self.tavv = Some(tavv.into());

        self
    }

    /// Sets Apple Pay as the payment type via an unencrypted token.
    pub fn apple_pay_tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::ApplePayDecrypted);
        self.tavv = Some(tavv.into());

        self
    }

    /// Sets Google Pay as the payment type via an unencrypted token.
    pub fn google_pay_tavv(mut self, tavv: impl Into<String>) -> Self {
        self.pay_type = Some(PayType::GooglePayDecrypted);
        self.tavv = Some(tavv.into());

        self
    }

    /// Sets the card CVV/CVV2 code.
    pub fn cvv(mut self, cvv: String) -> Self {
        self.card_cvv = Some(cvv);
        self
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

    /// Sets the identifier of the previous transaction, used for recurring payments with VISA tokens.
    pub fn tid(mut self, tid: String) -> Self {
        self.tid = Some(tid);
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

    /// Sets the operation to be performed without a client.
    pub fn recurring(mut self) -> Self {
        self.recurring = Some(true);
        self
    }

    /// Sets the API URL, where a notification is sent to on a payment status change.
    /// Must not exceed 510 characters.
    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    /// Sets the electronic commerce indicator, which identifies the type of a transaction
    /// and confirms the authentication via 3DS or another method.
    pub fn electronic_commerce_indicator(mut self, indicator: ElectronicCommerceIndicator) -> Self {
        self.electronic_commerce_indicator = Some(indicator);
        self
    }

    /// Sets the value to verify a cardholder's authenticity during 3DS processing.
    pub fn cardholder_authentication_verification_value(mut self, value: String) -> Self {
        self.cardholder_authentication_verification_value = Some(value);
        self
    }

    /// Sets the 3DS verification version.
    pub fn three_ds_version(mut self, version: String) -> Self {
        self.three_ds_version = Some(version);
        self
    }

    /// Sets the session identifier of a 3DS verification.
    pub fn three_ds_transaction_id(mut self, id: String) -> Self {
        self.three_ds_transaction_id = Some(id);
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

    /// Sets the sender's email.
    pub fn sender_email(mut self, email: String) -> Self {
        self.sender_email = Some(email);
        self
    }

    /// Sets the sender's country code in the ISO3166 format.
    pub fn sender_country_code(mut self, country: Country) -> Self {
        self.sender_country_code = Some(country.id.to_string());
        self
    }

    /// Sets the sender's city.
    pub fn sender_city(mut self, city: String) -> Self {
        self.sender_city = Some(city);
        self
    }

    /// Sets the sender's address.
    pub fn sender_address(mut self, address: String) -> Self {
        self.sender_address = Some(address);
        self
    }

    /// Sets the sender's state code in the ISO3166 format.
    pub fn sender_state(mut self, state: String) -> Self {
        self.sender_state = Some(state);
        self
    }

    /// Sets the sender's shipping state code in the ISO3166 format.
    pub fn sender_shipping_state(mut self, state: String) -> Self {
        self.sender_shipping_state = Some(state);
        self
    }

    /// Sets the sender's postal code.
    pub fn sender_postal_code(mut self, code: String) -> Self {
        self.sender_postal_code = Some(code);
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

    fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Hold,
            public_key: public_key.into(),
            amount,
            currency,
            order_id,
            description,
            card: None,
            card_exp_month: None,
            card_exp_year: None,
            apple_pay_token: None,
            google_pay_token: None,
            pay_type: None,
            electronic_commerce_indicator: None,
            cardholder_authentication_verification_value: None,
            three_ds_version: None,
            three_ds_transaction_id: None,
            card_cvv: None,
            ip: None,
            phone: None,
            tavv: None,
            tid: None,
            language: None,
            prepare: None,
            recurring_by_token: None,
            recurring: None,
            server_url: None,
            split_rules: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_email: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_state: None,
            sender_shipping_state: None,
            sender_postal_code: None,
            customer: None,
            detail_addenda: None,
            info: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FundsBlockingResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `reversed` - payment refunded, `success` - successful payment,
    /// `3ds_verify` - the verification by 3DS is required, `cvv_verify` - CVV required,
    /// `otp_verify` - confirmation by OTP is required, `receiver_verify` - additional receiver
    /// information is required, `sender_verify` - additional sender information is required,
    /// `hold_wait` - successful blocking of the amount on the sender's account,
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
    /// Represents the authorization code for debit.
    pub authcode_debit: Option<String>,
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
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
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

impl LiqPayResponse for FundsBlockingResponse {}

/// Represents a request to complete a payment with blocked funds.
#[derive(Debug, Serialize)]
pub struct PaymentCompletionRequest {
    version: Version,
    action: Action,
    public_key: String,
    amount: f64,
    order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rro_info: Option<RroInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    split_tickets_only: Option<bool>,
}

impl LiqPayRequest<PaymentCompletionResponse, Sha3_256> for PaymentCompletionRequest {}

impl PaymentCompletionRequest {
    pub fn new(public_key: impl Into<String>, amount: f64, order_id: String) -> Self {
        Self {
            version: Version::Seven,
            action: Action::HoldCompletion,
            public_key: public_key.into(),
            amount,
            order_id,
            rro_info: None,
            split_tickets_only: None,
        }
    }

    /// Sets a fiscalization data.
    pub fn rro_info(mut self, info: RroInfo) -> Self {
        self.rro_info = Some(info);
        self
    }

    /// Allows simulating a split without dividing the funds.
    pub fn split_tickets_only(mut self) -> Self {
        self.split_tickets_only = Some(true);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct PaymentCompletionResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error`, `failure`, `reversed`, `success`.
    pub status: Status,
    /// Represents the identifier of an acquirer.
    #[serde(rename = "acq_id")]
    pub acquirer_id: Option<u32>,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `success` - successful payment.
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
    /// Represents the sender's card token.
    pub card_token: Option<String>,
    /// Represents the commission charged to credit.
    pub commission_credit: Option<f64>,
    /// Represents the commission charged to debit.
    pub commission_debit: Option<f64>,
    /// Represents the date when functions were charged.
    pub completion_date: Option<String>,
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
    /// Represents the IP address of a sender.
    pub ip: Option<String>,
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

impl LiqPayResponse for PaymentCompletionResponse {}
