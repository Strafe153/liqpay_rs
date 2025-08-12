use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::{Action, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

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
    pub fn token_connect_control(
        public_key: impl Into<String>,
        is_debit: bool,
        push_account_receipt: String,
    ) -> Self {
        let mut request = Self::new(public_key, is_debit);
        request.push_account_receipt = Some(push_account_receipt);

        request
    }

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

#[derive(Debug, Deserialize)]
pub struct CardTokenInfo {
    #[serde(rename = "tokenRef")]
    pub token_ref: Option<String>,
    #[serde(rename = "tokenSuffix")]
    pub token_suffix: Option<String>,
    #[serde(rename = "tokenExpDate")]
    pub token_exp_date: Option<String>,
    pub status: Option<CardTokenInfoStatus>,
    pub decision: CardtokenInfoDescription,
}

#[derive(Debug, Deserialize)]
pub struct CreateTokenResponse {
    pub result: String,
    pub status: Option<String>,
    pub card_token: Option<String>,
    pub card_token_info: Option<CardTokenInfo>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CreateTokenResponse {}

#[derive(Debug, Serialize)]
pub enum CardTokenAction {
    #[serde(rename = "SUSPEND")]
    Suspend,
    #[serde(rename = "UNSUSPEND")]
    Unsuspend,
    #[serde(rename = "DELETE")]
    Delete,
}

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

#[derive(Debug, Deserialize)]
pub struct ChangeTokenStatusResponse {
    pub result: Result,
    pub status: Option<Status>,
    pub card_token: Option<String>,
    pub card_token_info: Option<CardTokenInfo>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for ChangeTokenStatusResponse {}
