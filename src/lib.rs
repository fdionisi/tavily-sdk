pub mod error;
pub mod search;

use std::sync::Arc;

use anyhow::{anyhow, Result};
use error::{HttpError, HttpErrorPayload, TavilyError};
use http_client::{http::Method, HttpClient, Request, RequestBuilderExt, ResponseAsyncBodyExt};
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Serialize};

pub const BASE_URL: &str = "https://api.tavily.com";

pub struct Tavily {
    http_client: Arc<dyn HttpClient>,
    api_key: SecretString,
    base_url: String,
}

pub struct TavilyBuilder {
    http_client: Option<Arc<dyn HttpClient>>,
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
            http_client: None,
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
            .http_client
            .send(
                Request::builder()
                    .uri(format!("{}{}", self.base_url, path.into()))
                    .method(Method::POST)
                    .json(&RequestWithApiKey {
                        api_key: &self.api_key.expose_secret(),
                        payload: serde_json::to_value(request).expect("couldn't serialize request"),
                    })?,
            )
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
    pub fn with_http_client(mut self, http_client: Arc<dyn HttpClient>) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub fn with_api_key<S>(mut self, api_key: S) -> Self
    where
        S: AsRef<str>,
    {
        self.api_key = Some(api_key.as_ref().to_string().into());
        self
    }

    pub fn with_base_url<S>(mut self, base_url: S) -> Self
    where
        S: AsRef<str>,
    {
        self.base_url = Some(base_url.as_ref().into());
        self
    }

    pub fn build(self) -> Result<Tavily> {
        Ok(Tavily {
            http_client: self.http_client.ok_or_else(|| anyhow!("http_client must be specified"))?,
            api_key: self.api_key.or_else(|| std::env::var("TAVILY_API_KEY").ok().map(SecretString::new))
                .ok_or_else(|| anyhow!("API key is required. Set it explicitly or use the TAVILY_API_KEY environment variable"))?,
            base_url: self.base_url.unwrap_or_else(|| BASE_URL.to_string()),
        })
    }
}
