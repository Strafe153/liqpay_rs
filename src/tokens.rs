use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents a token creation request.
#[derive(Debug, Serialize)]
pub struct CreateTokenRequest {
    version: Version,
    public_key: String,
    action: Action,
    is_debit: bool,
    is_credit: bool,
    #[serde(rename = "pushAccountReceipt", skip_serializing_if = "Option::is_none")]
    push_account_receipt: Option<String>,
    #[serde(rename = "pushData", skip_serializing_if = "Option::is_none")]
    push_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_month: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_exp_year: Option<String>,
    #[serde(rename = "expired_date", skip_serializing_if = "Option::is_none")]
    expiration_date: Option<String>,
}

impl LiqPayRequest<CreateTokenResponse, Sha3_256> for CreateTokenRequest {}

impl CreateTokenRequest {
    /// Constructs a new request to create a card token using Token connect control.
    pub fn token_connect_control(
        public_key: impl Into<String>,
        is_debit: bool,
        push_account_receipt: String,
    ) -> Self {
        let mut request = Self::new(public_key, is_debit);
        request.push_account_receipt = Some(push_account_receipt);

        request
    }

    /// Constructs a new request to create a card token using VISA cards enrollment hub.
    pub fn visa_cards_enrollment_hub(
        public_key: impl Into<String>,
        is_debit: bool,
        push_data: String,
        customer: String,
    ) -> Self {
        let mut request = Self::new(public_key, is_debit);

        request.push_data = Some(push_data);
        request.customer = Some(customer);

        request
    }

    /// Constructs a new request to create a card token using a card.
    pub fn card(
        public_key: impl Into<String>,
        is_debit: bool,
        card: String,
        cvv: String,
        exp_month: String,
        exp_year: String,
    ) -> Self {
        let mut request = Self::new(public_key, is_debit);

        request.card = Some(card);
        request.card_cvv = Some(cvv);
        request.card_exp_month = Some(exp_month);
        request.card_exp_year = Some(exp_year);

        request
    }

    /// Sets a token to be unique upon creation.
    pub fn unique(mut self, expiration_date: String) -> Self {
        self.action = Action::CreateUniqueToken;
        self.expiration_date = Some(expiration_date);

        self
    }

    fn new(public_key: impl Into<String>, is_debit: bool) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CreateToken,
            public_key: public_key.into(),
            is_debit,
            is_credit: !is_debit,
            push_data: None,
            customer: None,
            push_account_receipt: None,
            card: None,
            card_cvv: None,
            card_exp_month: None,
            card_exp_year: None,
            expiration_date: None,
        }
    }
}

/// Represents a card token information status.
#[derive(Debug, Deserialize)]
pub enum CardTokenInfoStatus {
    #[serde(rename = "INACTIVE")]
    Inactive,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "SUSPENDED")]
    Suspended,
    #[serde(rename = "DELETED")]
    Deleted,
}

/// Represents a card token information description.
#[derive(Debug, Deserialize)]
pub enum CardtokenInfoDescription {
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "REQUIRE_ADDITIONAL_AUTHENTICATION")]
    RequireAdditionalAuthentication,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ERROR")]
    Error,
}

/// Represents card token information.
#[derive(Debug, Deserialize)]
pub struct CardTokenInfo {
    /// Represents a token reference.
    #[serde(rename = "tokenRef")]
    pub token_ref: Option<String>,
    /// Represents a token suffix, which is the last 4 digits.
    #[serde(rename = "tokenSuffix")]
    pub token_suffix: Option<String>,
    /// Represents a token expiration date.
    #[serde(rename = "tokenExpDate")]
    pub token_exp_date: Option<String>,
    /// Represents a token status.
    pub status: Option<CardTokenInfoStatus>,
    /// Represents the payment decision regarding card digitization for MasterCard-only payments.
    pub decision: CardtokenInfoDescription,
}

/// Represents a token creation response.
#[derive(Debug, Deserialize)]
pub struct CreateTokenResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the operation.
    pub status: Option<Status>,
    /// Represents the generated card token.
    pub card_token: Option<String>,
    /// Represents the additional card token information.
    pub card_token_info: Option<CardTokenInfo>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CreateTokenResponse {}

/// Represents a card token action.
#[derive(Debug, Serialize)]
pub enum CardTokenAction {
    #[serde(rename = "SUSPEND")]
    Suspend,
    #[serde(rename = "UNSUSPEND")]
    Unsuspend,
    #[serde(rename = "DELETE")]
    Delete,
}

/// Represents a request to change a token status.
#[derive(Debug, Serialize)]
pub struct ChangeTokenStatusRequest {
    version: Version,
    public_key: String,
    action: Action,
    card_token: String,
    card_token_action: CardTokenAction,
}

impl LiqPayRequest<ChangeTokenStatusResponse, Sha3_256> for ChangeTokenStatusRequest {}

impl ChangeTokenStatusRequest {
    /// Constructs a new request to change a token status.
    pub fn new(
        public_key: impl Into<String>,
        card_token: String,
        card_token_action: CardTokenAction,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::UpdateToken,
            public_key: public_key.into(),
            card_token,
            card_token_action,
        }
    }
}

/// Represents a response of changing a token status.
#[derive(Debug, Deserialize)]
pub struct ChangeTokenStatusResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the operation.
    pub status: Option<Status>,
    /// Represents the generated card token.
    pub card_token: Option<String>,
    /// Represents the additional card token information.
    pub card_token_info: Option<CardTokenInfo>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for ChangeTokenStatusResponse {}
