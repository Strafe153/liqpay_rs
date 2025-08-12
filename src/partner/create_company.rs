use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha3::Sha3_256;

use crate::common::{Action, Currency, Language, Result, Status, Version};
use crate::traits::{LiqPayRequest, LiqPayResponse};

#[derive(Debug, Serialize)]
pub struct LawContacts {
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LawCtoInformation {
    pub name: Option<String>,
    pub inn: Option<String>,
    pub birth_date: Option<String>,
    pub citizenship: Option<String>,
    pub residency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LawOwnerInformation {
    pub name: Option<String>,
    pub inn: Option<String>,
    pub birth_date: Option<String>,
    pub citizenship: Option<String>,
    pub share_in_capital: Option<String>,
    pub residency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LawCoOwnerInformation {
    pub name: Option<String>,
    pub inn: Option<String>,
    pub birth_date: Option<String>,
    pub citizenship: Option<String>,
    pub share_in_capital: Option<String>,
    pub residency: Option<String>,
    pub company_name: Option<String>,
    pub company_okpo: Option<String>,
}

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

    pub fn amount_procent_agent(mut self, procent: f32) -> Self {
        self.amount_percent_agent = Some(procent);
        self
    }

    pub fn amount_static_agent(mut self, commission: f64, currency: Currency) -> Self {
        self.amount_static_agent = Some(commission);
        self.currency_static_agent = Some(currency);

        self
    }

    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    pub fn public_phone(mut self, phone: String) -> Self {
        self.public_phone = Some(phone);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyResponse {
    pub result: Result,
    pub status: Status,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for CreateCompanyResponse {}

#[derive(Debug, Serialize)]
pub struct Document {
    file: String,
    name: String,
    file_name: String,
    #[serde(rename = "doc_id", skip_serializing_if = "Option::is_none")]
    document_id: Option<String>,
}

impl Document {
    pub fn new(file: String, name: String, file_name: String) -> Self {
        Self {
            file,
            name,
            file_name,
            document_id: None,
        }
    }

    pub fn document_id(mut self, id: String) -> Self {
        self.document_id = Some(id);
        self
    }
}

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
    pub fn new(public_key: impl Into<String>) -> Self {
        Self {
            version: Version::Three,
            action: Action::MccCodes,
            public_key: public_key.into(),
            language: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Debug, Deserialize)]
pub struct MccCode {
    pub id: u32,
    pub mcc_code: Option<u32>,
    pub name: Option<String>,
    pub parent_id: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct MccCodesResponse {
    pub result: String,
    pub status: String,
    pub mcc_codes: Option<Vec<MccCode>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for MccCodesResponse {}

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
    pub fn new(public_key: impl Into<String>, mcc_code: u32) -> Self {
        Self {
            version: Version::Three,
            action: Action::MccCodes,
            public_key: public_key.into(),
            mcc_code,
            language: None,
        }
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }
}

#[derive(Debug, Deserialize)]
pub enum DocumentType {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "allof")]
    Allof,
    #[serde(rename = "optional")]
    Optional,
}

#[derive(Debug, Deserialize)]
pub struct MccDocument {
    pub doc_id: u32,
    pub doc_type: DocumentType,
    pub name: String,
    pub alt_docs: Vec<u32>,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct MccDocumentsResponse {
    pub result: String,
    pub status: String,
    pub mcc_docs: Option<Vec<MccDocument>>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for MccDocumentsResponse {}

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
    url_callback_status: Option<String>,
}

impl LiqPayRequest<RegisterCompanyResponse, Sha3_256> for RegisterCompanyRequest {}

impl RegisterCompanyRequest {
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

    pub fn site_url(mut self, url: String) -> Self {
        self.site_url = Some(url);
        self
    }

    pub fn iphone_app_url(mut self, url: String) -> Self {
        self.iphone_app_url = Some(url);
        self
    }

    pub fn android_app_url(mut self, url: String) -> Self {
        self.android_app_url = Some(url);
        self
    }

    pub fn telegram(mut self, url: String) -> Self {
        self.telegram = Some(url);
        self
    }

    pub fn viber(mut self, url: String) -> Self {
        self.viber = Some(url);
        self
    }

    pub fn instagram(mut self, url: String) -> Self {
        self.instagram = Some(url);
        self
    }

    pub fn facebook(mut self, url: String) -> Self {
        self.facebook = Some(url);
        self
    }

    pub fn documents(mut self, documents: Vec<Document>) -> Self {
        self.documents = Some(documents);
        self
    }

    pub fn enable_reports(mut self) -> Self {
        self.enable_reports = Some(String::from("true"));
        self
    }

    pub fn enable_checkout_edit(mut self) -> Self {
        self.enable_checkout_edit = Some(String::from("true"));
        self
    }

    pub fn logo(mut self, logo: String) -> Self {
        self.logo = Some(logo);
        self
    }

    pub fn url_callback_status(mut self, url: String) -> Self {
        self.url_callback_status = Some(url);
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
    pub result: Result,
    pub status: Option<Status>,
    #[serde(rename = "err_code")]
    pub error_code: Option<String>,
    #[serde(rename = "err_description")]
    pub error_description: Option<String>,
}

impl LiqPayResponse for RegisterCompanyResponse {}
