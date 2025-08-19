use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha3::Sha3_256;

use crate::common::enums::{Action, Currency, Language, Result, Status, Version};
use crate::common::traits::{LiqPayRequest, LiqPayResponse};

/// Represents the company's contact details
#[derive(Debug, Serialize)]
pub struct LawContacts {
    phone: Option<String>,
    email: Option<String>,
}

impl LawContacts {
    /// Construct an empty instance of contact details.
    pub fn new() -> Self {
        Self {
            phone: None,
            email: None,
        }
    }

    /// Sets the company's phone number.
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Sets the company's email.
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

/// Represents the company CTO's information.
#[derive(Debug, Serialize)]
pub struct LawCtoInformation {
    name: Option<String>,
    inn: Option<String>,
    birth_date: Option<String>,
    citizenship: Option<String>,
    residency: Option<String>,
}

impl LawCtoInformation {
    /// Construct an empty instance of the company CTO's information.
    pub fn new() -> Self {
        Self {
            name: None,
            inn: None,
            birth_date: None,
            citizenship: None,
            residency: None,
        }
    }

    /// Sets the company CTO's full name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the company CTO's TIN.
    pub fn inn(mut self, inn: String) -> Self {
        self.inn = Some(inn);
        self
    }

    /// Sets the company CTO's birth date in the YYYY-MM-DD format.
    pub fn birth_date(mut self, birth_date: String) -> Self {
        self.birth_date = Some(birth_date);
        self
    }

    /// Sets the company CTO's citizenship in latin.
    pub fn citizenship(mut self, citizenship: String) -> Self {
        self.citizenship = Some(citizenship);
        self
    }

    /// Sets the company CTO's place of residence.
    pub fn residency(mut self, residency: String) -> Self {
        self.residency = Some(residency);
        self
    }
}

/// Represents the company's owner information.
#[derive(Debug, Serialize)]
pub struct LawOwnerInformation {
    name: Option<String>,
    inn: Option<String>,
    birth_date: Option<String>,
    citizenship: Option<String>,
    share_in_capital: Option<String>,
    residency: Option<String>,
}

impl LawOwnerInformation {
    /// Construct an empty instance of the company owner's information.
    pub fn new() -> Self {
        Self {
            name: None,
            inn: None,
            birth_date: None,
            citizenship: None,
            residency: None,
            share_in_capital: None,
        }
    }

    /// Sets the company owner's full name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the company owner's TIN.
    pub fn inn(mut self, inn: String) -> Self {
        self.inn = Some(inn);
        self
    }

    /// Sets the company owner's birth date in the YYYY-MM-DD format.
    pub fn birth_date(mut self, birth_date: String) -> Self {
        self.birth_date = Some(birth_date);
        self
    }

    /// Sets the company owner's citizenship in latin.
    pub fn citizenship(mut self, citizenship: String) -> Self {
        self.citizenship = Some(citizenship);
        self
    }

    /// Sets the company owner's equity stake.
    pub fn share_in_capital(mut self, capital: String) -> Self {
        self.share_in_capital = Some(capital);
        self
    }

    /// Sets the company owner's place of residence.
    pub fn residency(mut self, residency: String) -> Self {
        self.residency = Some(residency);
        self
    }
}

/// Represents the company co-owner's information.
#[derive(Debug, Serialize)]
pub struct LawCoOwnerInformation {
    name: Option<String>,
    inn: Option<String>,
    birth_date: Option<String>,
    citizenship: Option<String>,
    share_in_capital: Option<String>,
    residency: Option<String>,
    company_name: Option<String>,
    company_okpo: Option<String>,
}

impl LawCoOwnerInformation {
    /// Construct an empty instance of the company co-owner's information.
    pub fn new() -> Self {
        Self {
            name: None,
            inn: None,
            birth_date: None,
            citizenship: None,
            residency: None,
            share_in_capital: None,
            company_name: None,
            company_okpo: None,
        }
    }

    /// Sets the company co-owner's full name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the company co-owner's TIN.
    pub fn inn(mut self, inn: String) -> Self {
        self.inn = Some(inn);
        self
    }

    /// Sets the company co-owner's birth date in the YYYY-MM-DD format.
    pub fn birth_date(mut self, birth_date: String) -> Self {
        self.birth_date = Some(birth_date);
        self
    }

    /// Sets the company co-owner's citizenship in latin.
    pub fn citizenship(mut self, citizenship: String) -> Self {
        self.citizenship = Some(citizenship);
        self
    }

    /// Sets the company co-owner's equity stake.
    pub fn share_in_capital(mut self, capital: String) -> Self {
        self.share_in_capital = Some(capital);
        self
    }

    /// Sets the company co-owner's place of residence.
    pub fn residency(mut self, residency: String) -> Self {
        self.residency = Some(residency);
        self
    }

    /// Sets the company's registered name. Must not exceed 255 characters.
    pub fn company_name(mut self, name: String) -> Self {
        self.company_name = Some(name);
        self
    }

    /// Sets the company's USREOU. Must not exceed 255 characters.
    pub fn company_okpo(mut self, okpo: String) -> Self {
        self.company_okpo = Some(okpo);
        self
    }
}

/// Represents a request to create a company.
#[derive(Debug, Serialize)]
pub struct CreateCompanyRequest {
    version: Version,
    public_key: String,
    action: Action,
    description: String,
    email: String,
    name: String,
    phone: String,
    site: String,
    iban: String,
    company: String,
    okpo: String,
    law_okpo: String,
    law_name: String,
    law_iban: String,
    law_contacts: LawContacts,
    law_cto_info: LawCtoInformation,
    law_owners_info: Vec<LawOwnerInformation>,
    law_co_owners_info: Vec<LawCoOwnerInformation>,
    #[serde(
        rename = "amount_procent_agent",
        skip_serializing_if = "Option::is_none"
    )]
    amount_percent_agent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_static_agent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_static_agent: Option<Currency>,
    #[serde(rename = "can_reports", skip_serializing_if = "Option::is_none")]
    enable_reports: Option<String>,
    #[serde(rename = "can_checkout_edit", skip_serializing_if = "Option::is_none")]
    enable_checkout_edit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    public_phone: Option<String>,
}

impl LiqPayRequest<CreateCompanyResponse, Sha3_256> for CreateCompanyRequest {}

impl CreateCompanyRequest {
    /// Construct a new request to create a company.
    pub fn new(
        public_key: impl Into<String>,
        description: String,
        email: String,
        name: String,
        phone: String,
        site: String,
        iban: String,
        company: String,
        okpo: String,
        law_okpo: String,
        law_name: String,
        law_iban: String,
        law_contacts: LawContacts,
        law_cto_info: LawCtoInformation,
        law_owners_info: Vec<LawOwnerInformation>,
        law_co_owners_info: Vec<LawCoOwnerInformation>,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::CreateShop,
            public_key: public_key.into(),
            description,
            email,
            name,
            phone,
            site,
            iban,
            company,
            okpo,
            law_okpo,
            law_name,
            law_iban,
            law_contacts,
            law_cto_info,
            law_owners_info,
            law_co_owners_info,
            amount_percent_agent: None,
            amount_static_agent: None,
            currency_static_agent: None,
            enable_reports: None,
            enable_checkout_edit: None,
            logo: None,
            public_phone: None,
        }
    }

    /// Sets the percentage of an agent fee.
    pub fn amount_percent_agent(mut self, percent: f32) -> Self {
        self.amount_percent_agent = Some(percent);
        self
    }

    /// Sets the static fee of an agent.
    pub fn amount_static_agent(mut self, commission: f64, currency: Currency) -> Self {
        self.amount_static_agent = Some(commission);
        self.currency_static_agent = Some(currency);

        self
    }

    /// Enables the payment history of the created company to be viewed by the owner.
    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    /// Enables the checkout page to be edited by the owner.
    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    /// Sets the URL to a company's logo.
    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Sets the public phone number of a company.
    pub fn public_phone(mut self, phone: String) -> Self {
        self.public_phone = Some(phone);
        self
    }
}

/// Represents a response to editing a company's information operation.
#[derive(Debug, Deserialize)]
pub struct CreateCompanyResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represents the private key of a company.
    pub private_key: Option<String>,
    /// Represents the public key of a company.
    pub public_key: Option<String>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CreateCompanyResponse {}

/// Represents a required document for a company activation in accordance with an MCC code.
#[derive(Debug, Serialize)]
pub struct Document {
    file: String,
    name: String,
    file_name: String,
    #[serde(rename = "doc_id", skip_serializing_if = "Option::is_none")]
    document_id: Option<String>,
}

impl Document {
    /// Construct a new instance of a document.
    pub fn new(file: String, name: String, file_name: String) -> Self {
        Self {
            file,
            name,
            file_name,
            document_id: None,
        }
    }

    /// Sets the identifier of a document.
    pub fn document_id(mut self, id: String) -> Self {
        self.document_id = Some(id);
        self
    }
}

/// Represents a request to get available MCC codes.
#[derive(Debug, Serialize)]
pub struct MccCodesRequest {
    action: Action,
    version: Version,
    public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
}

impl LiqPayRequest<MccCodesResponse, Sha1> for MccCodesRequest {}

impl MccCodesRequest {
    /// Construct a new request to get available MCC codes.
    pub fn new(public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Three,
            action: Action::MccCodes,
            public_key: public_key.into(),
            language: None,
        }
    }

    /// Sets the language of the returned MCC codes.
    /// Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

/// Represents an MCC code.
#[derive(Debug, Deserialize)]
pub struct MccCode {
    /// Represents the identifier of a code.
    pub id: u32,
    /// Represents the code itself.
    pub mcc_code: Option<u32>,
    /// Represents the name of the code.
    pub name: Option<String>,
    /// Represents the identifier of the category a code belongs to.
    pub parent_id: Option<u32>,
}

/// Represents a response to getting available MCC codes operation.
#[derive(Debug, Deserialize)]
pub struct MccCodesResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represents a collection of available MCC codes.
    pub mcc_codes: Option<Vec<MccCode>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for MccCodesResponse {}

/// Represents a request to get documents for an MCC code.
#[derive(Debug, Serialize)]
pub struct MccDocumentsRequest {
    action: Action,
    version: Version,
    public_key: String,
    mcc_code: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    language: Option<Language>,
}

impl LiqPayRequest<MccDocumentsResponse, Sha1> for MccDocumentsRequest {}

impl MccDocumentsRequest {
    /// Construct a new request to get documents for an MCC codes.
    pub fn new(public_key: impl Into<String>, mcc_code: u32) -> Self {
        Self {
            version: Version::Three,
            action: Action::MccCodes,
            public_key: public_key.into(),
            mcc_code,
            language: None,
        }
    }

    /// Sets the language of the returned MCC codes.
    /// Allowed values are `uk` - Ukrainian and `en` - English.
    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

/// Represents a document type.
#[derive(Debug, Deserialize)]
pub enum DocumentType {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "allof")]
    Allof,
    #[serde(rename = "optional")]
    Optional,
}

/// Represents an MCC document.
#[derive(Debug, Deserialize)]
pub struct MccDocument {
    /// Represents the identifier of a document.
    pub doc_id: u32,
    /// Represents the type of a document.
    pub doc_type: DocumentType,
    /// Represents the name of a document.
    pub name: String,
    /// Represents a collection of identifier of documents that can be used as replacements
    /// for the current document.
    pub alt_docs: Vec<u32>,
    /// Represents the description of a document.
    pub description: String,
}

/// Represents a response to getting documents for an MCC code operation.
#[derive(Debug, Deserialize)]
pub struct MccDocumentsResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Status,
    /// Represents a collection of an MCC code documents.
    pub mcc_docs: Option<Vec<MccDocument>>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for MccDocumentsResponse {}

#[derive(Debug, Serialize)]
pub enum UrlCallbackStatus {
    #[serde(rename = "5")]
    Activated,
    #[serde(rename = "6")]
    Blocked,
}

/// Represents a request to register a company.
#[derive(Debug, Serialize)]
pub struct RegisterCompanyRequest {
    version: Version,
    public_key: String,
    action: Action,
    phone: String,
    email: String,
    name: String,
    iban: String,
    okpo: String,
    company: String,
    mcc_code: String,
    law_okpo: String,
    law_name: String,
    law_iban: String,
    law_contacts: LawContacts,
    law_cto_info: LawCtoInformation,
    law_owners_info: Vec<LawOwnerInformation>,
    law_co_owners_info: Vec<LawCoOwnerInformation>,
    #[serde(rename = "url_site", skip_serializing_if = "Option::is_none")]
    site_url: Option<String>,
    #[serde(rename = "url_app_iphone", skip_serializing_if = "Option::is_none")]
    iphone_app_url: Option<String>,
    #[serde(rename = "url_app_android", skip_serializing_if = "Option::is_none")]
    android_app_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    telegram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    viber: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instagram: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    facebook: Option<String>,
    #[serde(rename = "docs", skip_serializing_if = "Option::is_none")]
    documents: Option<Vec<Document>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_percent_agent: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    amount_static_agent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency_static_agent: Option<Currency>,
    #[serde(rename = "can_reports", skip_serializing_if = "Option::is_none")]
    enable_reports: Option<String>,
    #[serde(rename = "can_checkout_edit", skip_serializing_if = "Option::is_none")]
    enable_checkout_edit: Option<String>,
    logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_callback_status: Option<UrlCallbackStatus>,
}

impl LiqPayRequest<RegisterCompanyResponse, Sha3_256> for RegisterCompanyRequest {}

impl RegisterCompanyRequest {
    /// Construct a new request to register a company with commission specified as a percentage.
    pub fn with_percent_commission(
        public_key: impl Into<String>,
        email: String,
        name: String,
        phone: String,
        iban: String,
        company: String,
        okpo: String,
        mcc_code: String,
        law_okpo: String,
        law_name: String,
        law_iban: String,
        law_contacts: LawContacts,
        law_cto_info: LawCtoInformation,
        law_owners_info: Vec<LawOwnerInformation>,
        law_co_owners_info: Vec<LawCoOwnerInformation>,
        amount_percent_agent: f32,
    ) -> Self {
        let mut request = Self::new(
            public_key,
            email,
            name,
            phone,
            iban,
            company,
            okpo,
            mcc_code,
            law_okpo,
            law_name,
            law_iban,
            law_contacts,
            law_cto_info,
            law_owners_info,
            law_co_owners_info,
        );

        request.amount_percent_agent = Some(amount_percent_agent);

        request
    }

    /// Construct a new request to register a company with a static commission.
    pub fn with_static_commission(
        public_key: impl Into<String>,
        email: String,
        name: String,
        phone: String,
        iban: String,
        company: String,
        okpo: String,
        mcc_code: String,
        law_okpo: String,
        law_name: String,
        law_iban: String,
        law_contacts: LawContacts,
        law_cto_info: LawCtoInformation,
        law_owners_info: Vec<LawOwnerInformation>,
        law_co_owners_info: Vec<LawCoOwnerInformation>,
        amount_static_commission: f64,
        currency_static_agent: Currency,
    ) -> Self {
        let mut request = Self::new(
            public_key,
            email,
            name,
            phone,
            iban,
            company,
            okpo,
            mcc_code,
            law_okpo,
            law_name,
            law_iban,
            law_contacts,
            law_cto_info,
            law_owners_info,
            law_co_owners_info,
        );

        request.amount_static_agent = Some(amount_static_commission);
        request.currency_static_agent = Some(currency_static_agent);

        request
    }

    /// Sets the URL to the company's website.
    pub fn site_url(mut self, url: String) -> Self {
        self.site_url = Some(url);
        self
    }

    /// Sets the URL to the company's App Store application.
    pub fn iphone_app_url(mut self, url: String) -> Self {
        self.iphone_app_url = Some(url);
        self
    }

    /// Sets the URL to the company's Play Market application.
    pub fn android_app_url(mut self, url: String) -> Self {
        self.android_app_url = Some(url);
        self
    }

    /// Sets the URL to the company's Telegram bot.
    pub fn telegram(mut self, url: String) -> Self {
        self.telegram = Some(url);
        self
    }

    /// Sets the URL to the company's Viber bot.
    pub fn viber(mut self, url: String) -> Self {
        self.viber = Some(url);
        self
    }

    /// Sets the URL to the company's Instagram profile.
    pub fn instagram(mut self, url: String) -> Self {
        self.instagram = Some(url);
        self
    }

    /// Sets the URL to the company's Facebook profile.
    pub fn facebook(mut self, url: String) -> Self {
        self.facebook = Some(url);
        self
    }

    /// Sets the list of required documents for a company registration.
    pub fn documents(mut self, documents: Vec<Document>) -> Self {
        self.documents = Some(documents);
        self
    }

    /// Enables the payment history of the created company to be viewed by the owner.
    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    /// Enables the checkout page to be edited by the owner.
    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    /// Sets the URL to a company's logo.
    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Sets the callback URL for getting information when the store status changes.
    pub fn url_callback_status(mut self, status: UrlCallbackStatus) -> Self {
        self.url_callback_status = Some(status);
        self
    }

    fn new(
        public_key: impl Into<String>,
        email: String,
        name: String,
        phone: String,
        iban: String,
        company: String,
        okpo: String,
        mcc_code: String,
        law_okpo: String,
        law_name: String,
        law_iban: String,
        law_contacts: LawContacts,
        law_cto_info: LawCtoInformation,
        law_owners_info: Vec<LawOwnerInformation>,
        law_co_owners_info: Vec<LawCoOwnerInformation>,
    ) -> Self {
        Self {
            version: Version::Seven,
            action: Action::RegisterShop,
            public_key: public_key.into(),
            email,
            name,
            phone,
            iban,
            company,
            okpo,
            mcc_code,
            law_okpo,
            law_name,
            law_iban,
            law_contacts,
            law_cto_info,
            law_owners_info,
            law_co_owners_info,
            site_url: None,
            iphone_app_url: None,
            android_app_url: None,
            telegram: None,
            viber: None,
            instagram: None,
            facebook: None,
            documents: None,
            amount_percent_agent: None,
            amount_static_agent: None,
            currency_static_agent: None,
            enable_reports: None,
            enable_checkout_edit: None,
            logo: None,
            url_callback_status: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterCompanyResponse {
    /// Represents the result of the request. Can be either `ok` or `error`.
    pub result: Result,
    /// Represents the status of the request.
    pub status: Option<Status>,
    /// Holds an error code.
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    /// Holds an error description.
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RegisterCompanyResponse {}
