use serde::Serialize;

/// Represents a report format in a response.
#[derive(Debug, Serialize)]
pub enum ResponseFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "xml")]
    Xml,
}
