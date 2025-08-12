use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum Result {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Serialize)]
pub enum ResponseFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "xml")]
    Xml,
}
