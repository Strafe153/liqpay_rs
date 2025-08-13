use std::error::Error;

use base64::Engine;
use base64::engine::general_purpose;
use reqwest::Client;
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha1::Digest;

use crate::common::traits::{LiqPayRequest, LiqPayResponse};

const CLIENT_URL: &'static str = "https://www.liqpay.ua/api/request";
const DATA: &'static str = "data";
const SIGNATURE: &'static str = "signature";

type BoxError = Box<dyn Error + Send + Sync>;
type FormData = [(&'static str, String); 2];

fn build_signature(private_key: &str, body: &str) -> String {
    format!("{}{}{}", private_key, body, private_key)
}

fn build_form_data<Req, Resp, Alg>(private_key: &str, request: Req) -> Result<FormData, BoxError>
where
    Req: LiqPayRequest<Resp, Alg> + Serialize,
    Resp: LiqPayResponse + DeserializeOwned,
    Alg: Digest,
{
    let serialized_request = serde_json::to_string(&request)?;
    let encoded_request = general_purpose::STANDARD.encode(serialized_request.as_bytes());

    let signature = build_signature(private_key, &encoded_request);
    let hashed_signature = Alg::digest(signature.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(hashed_signature);

    let form_data = [(DATA, encoded_request), (SIGNATURE, encoded_signature)];

    Ok(form_data)
}

pub struct LiqPayClient {
    client: Client,
    private_key: String,
}

impl LiqPayClient {
    pub fn new(private_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            private_key: private_key.into(),
        }
    }

    pub async fn send<Req, Resp, Alg>(&self, request: Req) -> Result<Resp, BoxError>
    where
        Req: LiqPayRequest<Resp, Alg> + Serialize,
        Resp: LiqPayResponse + DeserializeOwned,
        Alg: Digest,
    {
        let form_data = build_form_data(&self.private_key, request)?;

        let deserialized_response = self
            .client
            .post(CLIENT_URL)
            .form(&form_data)
            .send()
            .await?
            .json()
            .await?;

        Ok(deserialized_response)
    }
}

#[cfg(feature = "blocking")]
pub mod blocking {
    use super::*;

    use reqwest::blocking::Client;

    pub struct BlockLiqPayClient {
        client: Client,
        private_key: String,
    }

    impl BlockLiqPayClient {
        pub fn new(private_key: impl Into<String>) -> Self {
            Self {
                client: Client::new(),
                private_key: private_key.into(),
            }
        }

        pub fn send<Req, Resp, Alg>(&self, request: Req) -> Result<Resp, BoxError>
        where
            Req: LiqPayRequest<Resp, Alg> + Serialize,
            Resp: LiqPayResponse + DeserializeOwned,
            Alg: Digest,
        {
            let form_data = build_form_data(&self.private_key, request)?;

            let deserialized_response = self
                .client
                .post(CLIENT_URL)
                .form(&form_data)
                .send()?
                .json()?;

            Ok(deserialized_response)
        }
    }
}
