use std::collections::HashMap;
use std::error::Error;

use base64::Engine;
use base64::engine::general_purpose;
use reqwest::{Client, blocking::Client as BlockingClient};
use serde::{Serialize, de::DeserializeOwned};
use sha1::Digest;

const CLIENT_URL: &'static str = "https://www.liqpay.ua/api/request";
const DATA: &'static str = "data";
const SIGNATURE: &'static str = "signature";

type BoxError = Box<dyn Error + Send + Sync>;

// fn test<T>(request: &T, private_key: &str) -> Result<HashMap<&'static str, String>, Box<dyn Error>>
// where
//     T: LiqPayRequest<LiqpayResponse> + Serialize
// {
//     let serialized_json = serde_json::to_string(request)?;
//     let b64_json = general_purpose::STANDARD.encode(serialized_json.as_bytes());

//     // for testing purposes
//     // println!("{}", serialized_json);

//     let signature = format!("{}{}{}", private_key, b64_json, private_key);

//     let hashed_signature = Sha1::new()
//         .chain_update(signature.as_bytes())
//         .finalize();

//     let b64_hashed_signature = general_purpose::STANDARD.encode(hashed_signature);

//     let form_data: HashMap<&'static str, String> = HashMap::from([
//         (DATA, b64_json),
//         (SIGNATURE, b64_hashed_signature)
//     ]);

//     return Ok(form_data);
// }

fn form_signature(private_key: impl Into<String>, body: &str) -> String {
    let private_key = private_key.into();

    format!("{}{}{}", private_key, body, private_key)
}

pub trait LiqPayRequest<R, H>
where
    R: LiqPayResponse + DeserializeOwned,
    H: Digest,
{
    fn send(&self, private_key: impl Into<String>) -> Result<String, BoxError>
    where
        Self: Serialize,
    {
        let serialized_request = serde_json::to_string(self)?;
        let encoded_request = general_purpose::STANDARD.encode(serialized_request.as_bytes());

        let signature = form_signature(private_key, &encoded_request);
        let hashed_signature = H::digest(signature.as_bytes());
        let encoded_signature = general_purpose::STANDARD.encode(hashed_signature);

        let form_data: HashMap<&'static str, String> =
            HashMap::from([(DATA, encoded_request), (SIGNATURE, encoded_signature)]);

        let temp = BlockingClient::new()
            .post(CLIENT_URL)
            .form(&form_data)
            .send()?;

        let deserialized_response = temp.text()?;

        Ok(deserialized_response)
    }

    fn send_async(
        &self,
        private_key: impl Into<String>,
    ) -> impl Future<Output = Result<R, BoxError>>
    where
        Self: Serialize,
    {
        async move {
            let serialized_request = serde_json::to_string(self)?;
            let encoded_request = general_purpose::STANDARD.encode(serialized_request.as_bytes());

            let signature = form_signature(private_key, &encoded_request);
            let hashed_signature = H::digest(signature.as_bytes());
            let encoded_signature = general_purpose::STANDARD.encode(hashed_signature);

            let form_data: HashMap<&'static str, String> =
                HashMap::from([(DATA, encoded_request), (SIGNATURE, encoded_signature)]);

            let deserialized_response = Client::new()
                .post(CLIENT_URL)
                .form(&form_data)
                .send()
                .await?
                .json::<R>()
                .await?;

            Ok(deserialized_response)
        }
    }
}

pub trait LiqPayResponse {}
