use iso3166::Country;
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, MpiEci, Prepare, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a request to perform a money transfer from a company account to a beneficiary.
#[derive(Debug, Serialize)]
pub struct P2PDebitRequest {
    version: Version,
    public_key: String,
    action: Action,
    amount: f64,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_year: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prepare: Option<Prepare>,
    #[serde(rename = "recurringbytoken", skip_serializing_if = "Option::is_none")]
    recurring_by_token: Option<char>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sandbox: Option<char>,
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
    mpi_eci: Option<MpiEci>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpi_cres: Option<String>,
}

impl LiqPayRequest<P2PDebitResponse, Sha3_256> for P2PDebitRequest {}

impl P2PDebitRequest {
    /// Constructs a new request to perform a money transfer by a card number.
    pub fn by_card(
        public_key: impl Into<String>,
        amount: f64,
        card: String,
        cvv: String,
        exp_month: String,
        exp_year: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);

        request.card = Some(card);
        request.card_cvv = Some(cvv);
        request.card_exp_month = Some(exp_month);
        request.card_exp_year = Some(exp_year);

        request
    }

    /// Constructs a new request to perform a money transfer by a card token.
    pub fn by_token(
        public_key: impl Into<String>,
        amount: f64,
        card_token: String,
        currency: Currency,
        order_id: String,
        description: String,
    ) -> Self {
        let mut request = Self::new(public_key, amount, currency, order_id, description);
        request.card_token = Some(card_token);

        request
    }

    /// Sets the customer's phone number.
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Sets the customer's language. Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn langugae(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Sets the preliminary preparation of the payment. The mode allows to determine data completeness,
    /// whether 3DS is required or limit is exceeded. The funds are not debited.
    pub fn prepare(mut self, prepare: Prepare) -> Self {
        self.prepare = Some(prepare);
        self
    }

    /// Sets the payer `card_token` generation, which is then received in the `server_url`.
    pub fn recurring_by_token(mut self) -> Self {
        self.recurring_by_token = Some('1');
        self
    }

    /// Sets the URL of the shop, to which the customer is redirected after completing a purchase.
    /// Must not exceed 510 characters.
    pub fn result_url(mut self, url: String) -> Self {
        self.result_url = Some(url);
        self
    }

    /// Sets the API URL, where a notification is sent to on a payment status change.
    /// Must not exceed 510 characters.
    pub fn server_url(mut self, url: String) -> Self {
        self.server_url = Some(url);
        self
    }

    /// Sets the sandbo environment for testing purposes.
    pub fn sandbox(mut self) -> Self {
        self.sandbox = Some('1');
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

    /// Sets the MPI ECI code.
    pub fn mpi_eci(mut self, eci: MpiEci) -> Self {
        self.mpi_eci = Some(eci);
        self
    }

    /// Sets ACS value.
    pub fn mpi_cres(mut self, cres: String) -> Self {
        self.mpi_cres = Some(cres);
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
            action: Action::P2PDebit,
            public_key: public_key.into(),
            amount,
            card: None,
            card_cvv: None,
            card_exp_month: None,
            card_exp_year: None,
            card_token: None,
            currency,
            order_id,
            description,
            phone: None,
            language: None,
            prepare: None,
            recurring_by_token: None,
            result_url: None,
            server_url: None,
            sandbox: None,
            sender_first_name: None,
            sender_last_name: None,
            sender_email: None,
            sender_country_code: None,
            sender_city: None,
            sender_address: None,
            sender_state: None,
            sender_shipping_state: None,
            sender_postal_code: None,
            mpi_eci: None,
            mpi_cres: None,
        }
    }
}

/// Represents the response to a money transferring operation from a company account to a beneficiary.
#[derive(Debug, Deserialize)]
pub struct P2PDebitResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request. Possible values are `error` - incorrect data,
    /// `failure` - payment failed, `success` - successful payment.
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
    /// Represents the receiver's commission.
    pub receiver_commission: Option<f32>,
    /// Represents the 3DS URL for payment confirmation.
    pub redirect_to: Option<String>,
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

impl LiqPayResponse for P2PDebitResponse {}
