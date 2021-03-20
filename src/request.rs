use crate::{Error, TwitterAPI, TwitterAPIErrorMessage};
use anyhow::{Context, Result};
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Method,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

impl TwitterAPI {
    pub(crate) async fn request(
        &self,
        endpoint: &str,
        method: reqwest::Method,
        params: &HashMap<&str, String>,
    ) -> Result<String, Error> {
        let mut params_ref: HashMap<&str, &str> = maplit::hashmap! {};
        for (&k, v) in params {
            params_ref.insert(k, v);
        }
        let token = self.create_oauth_header(endpoint, &method.to_string(), &params_ref);
        let parsed_token = token
            .parse::<HeaderValue>()
            .map_err(|source| Error::Error(source.into()))?;

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, parsed_token);

        self.client
            .request(method, endpoint)
            .query(&params_ref)
            .headers(headers)
            .send()
            .await
            .map_err(|source| Error::HTTPRequestError(source))
            .with_context(|| "HTTP request failed.")?
            .text()
            .await
            .with_context(|| "Response body is invalid.")
            .map_err(|source| Error::Error(source.into()))
    }

    pub async fn raw_get<T>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let text = self.request(endpoint, Method::GET, params).await?;

        serde_json::from_str::<T>(&text).or_else(|e| {
            match serde_json::from_str::<TwitterAPIErrorMessage>(&text) {
                Ok(v) => Err(Error::TwitterAPIError(v, format!("{:?}", params))),
                Err(_) => Err(Error::JsonParsingError(e.into(), text)),
            }
        })
    }

    pub async fn raw_post<T>(
        &self,
        endpoint: &str,
        params: &HashMap<&str, String>,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let text = self.request(endpoint, Method::POST, params).await?;
        serde_json::from_str::<T>(&text).or_else(|e| {
            match serde_json::from_str::<TwitterAPIErrorMessage>(&text) {
                Ok(v) => Err(Error::TwitterAPIError(v, format!("{:?}", params))),
                Err(_) => Err(Error::JsonParsingError(e.into(), text)),
            }
        })
    }
}
