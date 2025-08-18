use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents the depth of a browser color.
#[derive(Debug, Serialize)]
pub enum BrowserColorDepth {
    #[serde(rename = "1")]
    One,
    #[serde(rename = "2")]
    Two,
    #[serde(rename = "4")]
    Four,
    #[serde(rename = "8")]
    Eight,
    #[serde(rename = "15")]
    Fifteen,
    #[serde(rename = "16")]
    Sixteen,
    #[serde(rename = "24")]
    TwentyFour,
    #[serde(rename = "32")]
    ThirtyTwo,
    #[serde(rename = "48")]
    FortyEight,
}

/// Represents additional 3D Secure information.
#[derive(Debug, Serialize)]
pub struct ThreeDsInfo {
    #[serde(rename = "notificationURL")]
    notification_url: String,
    #[serde(rename = "browserLanguage")]
    browser_language: String,
    #[serde(rename = "threeDSRequestorURL")]
    three_ds_requestor_url: String,
    #[serde(rename = "browserScreenHeight")]
    browser_screen_height: String,
    #[serde(rename = "browserColorDepth")]
    browser_color_depth: BrowserColorDepth,
    #[serde(rename = "browserScreenWidth")]
    browser_screen_width: String,
    #[serde(rename = "browserAcceptHeader")]
    browser_accept_header: String,
    #[serde(rename = "browserTZ")]
    browser_tz: i16,
    #[serde(rename = "browserUserAgent")]
    browser_user_agent: String,
    #[serde(rename = "browserJavascriptEnabled")]
    browser_java_script_enabled: Option<bool>,
    #[serde(rename = "browserJavaEnabled")]
    browser_java_enabled: Option<bool>,
}

impl ThreeDsInfo {
    /// Constructs a new instance of additional 3D Secure information.
    pub fn new(
        notification_url: String,
        browser_language: String,
        three_ds_requestor_url: String,
        browser_screen_height: String,
        browser_screen_width: String,
        browser_color_depth: BrowserColorDepth,
        browser_accept_header: String,
        browser_tz_compared_to_utc: i8,
        browser_user_agent: String,
    ) -> Self {
        let minutes_in_hour: u8 = 60;
        let browser_tz = (browser_tz_compared_to_utc as i16) * -(minutes_in_hour as i16);

        Self {
            notification_url,
            browser_language,
            three_ds_requestor_url,
            browser_screen_height,
            browser_screen_width,
            browser_color_depth,
            browser_accept_header,
            browser_tz,
            browser_user_agent,
            browser_java_script_enabled: None,
            browser_java_enabled: None,
        }
    }

    /// Sets JavaScript execution in a customer's browser to disabled.
    pub fn disable_java_script(mut self) -> Self {
        self.browser_java_enabled = Some(false);
        self
    }

    /// Sets Java execution in a customer's browser to enabled.
    pub fn enable_java(mut self) -> Self {
        self.browser_java_enabled = Some(true);
        self
    }
}

/// Represents a request to verify a card for 3DS support.
#[derive(Debug, Serialize)]
pub struct MpiRequest {
    version: Version,
    action: Action,
    public_key: String,
    amount: f64,
    card: String,
    card_exp_month: String,
    card_exp_year: String,
    currency: Currency,
    order_id: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action_payment: Option<Action>,
    language: Option<Language>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sender_last_name: Option<String>,
    #[serde(rename = "threeDSInfo", skip_serializing_if = "Option::is_none")]
    three_ds_info: Option<ThreeDsInfo>,
}

impl LiqPayRequest<MpiResponse, Sha3_256> for MpiRequest {}

impl MpiRequest {
    /// Constructs a new request to verify a card for 3DS support.
    pub fn new(
        public_key: impl Into<String>,
        amount: f64,
        currency: Currency,
        card: String,
        exp_month: String,
        exp_year: String,
        order_id: String,
        description: String,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::Mpi,
            public_key: public_key.into(),
            amount,
            currency,
            card,
            card_exp_year: exp_year,
            card_exp_month: exp_month,
            order_id,
            description,
            card_cvv: None,
            email: None,
            ip: None,
            action_payment: None,
            language: None,
            sender_first_name: None,
            sender_last_name: None,
            three_ds_info: None,
        }
    }

    /// Sets the card CVV/CVV2 code.
    pub fn cvv(mut self, cvv: String) -> Self {
        self.card_cvv = Some(cvv);
        self
    }

    /// Sets the email of a customer.
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
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

    /// Sets a payment action to `hold`.
    pub fn hold_action(mut self) -> Self {
        self.action_payment = Some(Action::Hold);
        self
    }

    /// Sets a payment action to `subscribe`.
    pub fn subscribe_action(mut self) -> Self {
        self.action_payment = Some(Action::Subscribe);
        self
    }

    /// Sets a payment action to `paydonate`.
    pub fn pay_donate_action(mut self) -> Self {
        self.action_payment = Some(Action::PayDonate);
        self
    }

    /// Sets a payment action to `auth`.
    pub fn auth_action(mut self) -> Self {
        self.action_payment = Some(Action::Auth);
        self
    }

    /// Sets a payment action to `p2p`.
    pub fn p2p_action(mut self) -> Self {
        self.action_payment = Some(Action::P2P);
        self
    }

    /// Sets a payment action to `p2pdebit`.
    pub fn p2p_debit(mut self) -> Self {
        self.action_payment = Some(Action::P2PDebit);
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

    /// Sets additional 3D Secure information.
    pub fn three_ds_info(mut self, info: ThreeDsInfo) -> Self {
        self.three_ds_info = Some(info);
        self
    }
}

/// Represents an MPI status.
#[derive(Debug, Deserialize)]
pub enum MpiStatus {
    /// In 3DS Version 1.0 represents the support for 3D Secure,
    /// in Version 2.0 - no additional verification required.
    Y,
    /// In Version 2.0 means no additional verification required.
    A,
    /// in Version 2.0 means that required additional verification.
    C,
    /// In 3DS Version 1.0 represents the lack of required 3DS verification,
    /// in Version 2.0 means client verification failed.
    N,
    /// In both 3DS Version 1.0 and  2.0 represents the failed card verification.
    U,
}

/// Represents the response to a 3D Secure card verification operation.
#[derive(Debug, Deserialize)]
pub struct MpiResponse {
    /// Represents the status of the request.
    /// Possible values are `error` - incorrect data, `failure` - payment failed.
    pub status: Status,
    /// Represents the result of the request.
    pub result: Option<Result>,
    /// Represents a required parameter for ACS authentication.
    pub mpi_req_md: Option<String>,
    /// Represents a required parameter for ACS authentication.
    pub mpi_re1q_pareq: Option<String>,
    /// Represents a #D Secure confirmation page URL. 
    pub mpi_req_url: Option<String>,
    /// Represents a status of 3D Secure verification. 
    pub mpi_status: Option<MpiStatus>,
    /// Contains the value "2.0" if the second version is supported, otherwise empty.
    pub mpi_version: Option<String>,
    /// Represents the authentication form if the second version is supported.
    pub mpi_form: Option<String>,
    /// Represents the required parameter for authentication for N and Y statuses.
    pub mpi_cres: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for MpiResponse {}
