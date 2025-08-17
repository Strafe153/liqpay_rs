use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ResponseFormat {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "xml")]
    Xml,
}
