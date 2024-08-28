mod error;
mod search;

use anyhow::{anyhow, Result};
use error::{HttpError, HttpErrorPayload, TavilyError};
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Serialize};

pub const BASE_URL: &str = "https://api.tavily.com";

pub struct Tavily {
    client: reqwest::Client,
    api_key: SecretString,
    base_url: String,
}

pub struct TavilyBuilder {
    api_key: Option<SecretString>,
    base_url: Option<String>,
}

#[derive(Serialize)]
struct RequestWithApiKey<'a> {
    api_key: &'a String,
    #[serde(flatten)]
    payload: serde_json::Value,
}

impl Tavily {
    pub fn builder() -> TavilyBuilder {
        TavilyBuilder {
            api_key: None,
            base_url: None,
        }
    }

    pub(crate) async fn post<P, S, D>(&self, path: P, request: S) -> Result<D, TavilyError>
    where
        P: Into<String>,
        S: Serialize,
        D: DeserializeOwned,
    {
        let response = self
            .client
            .post(format!("{}{}", self.base_url, path.into()))
            .json(&RequestWithApiKey {
                api_key: self.api_key.expose_secret(),
                payload: serde_json::to_value(request).expect("couldn't serialize request"),
            })
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let payload = response.json::<HttpErrorPayload>().await?;
            return Err(TavilyError::HttpError(HttpError {
                status: status.as_u16(),
                payload,
            }));
        }

        let response = response.json::<D>().await?;
        Ok(response)
    }
}

impl TavilyBuilder {
    pub fn api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    pub fn base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    pub fn build(self) -> Result<Tavily> {
        Ok(Tavily {
            client: reqwest::Client::new(),
            api_key: self.api_key.or_else(|| std::env::var("TAVILY_API_KEY").ok().map(SecretString::new))
                .ok_or_else(|| anyhow!("API key is required. Set it explicitly or use the TAVILY_API_KEY environment variable"))?,
            base_url: self.base_url.unwrap_or_else(|| BASE_URL.to_string()),
        })
    }
}
