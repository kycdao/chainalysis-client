mod error;
mod types;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

pub use error::*;
pub use json_api::*;
pub use types::*;

const API_URL: &str = "https://public.chainalysis.com/api/v1/";

pub struct Client {
    api: ApiClient,
}

impl Client {
    pub fn new(api_key: &str) -> Result<Client> {
        let header_key = HeaderValue::from_str(api_key).expect("Invalid API key value");

        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-api-key"), header_key);

        let client = ApiClient::new(API_URL, AuthConfig::NoAuth, Some(headers))?;

        Ok(Client { api: client })
    }

    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: JsonResponse,
    {
        self.api.get(path, None, None).await.map_err(Error::from)
    }

    pub async fn get_address_sanctions(&self, address: &str) -> Result<AddressSanctions> {
        let path = format!("address/{}", address);
        self.get(&path).await
    }

    pub async fn is_sanctioned(&self, address: &str) -> Result<bool> {
        let sanctions = self.get_address_sanctions(address).await?;
        let is_sanctioned = sanctions.identifications.len() > 0;
        Ok(is_sanctioned)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    // put unittests here
}
