use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
};
use std::{result, time::Duration};

use crate::specification::api_spec::ApiConfig;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(config: ApiConfig) -> Self{
        let mut headers = HeaderMap::new();

        headers.insert(
            CONTENT_TYPE, 
            HeaderValue::from_static("application/json")
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self { 
            client, 
            base_url: config.base_url 
        }
    }

    pub fn get(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}{}", self.base_url, path))
    }
    
    pub fn post(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .post(format!("{}{}", self.base_url, path))
    }

    pub fn put(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .put(format!("{}{}", self.base_url, path))
    }

    pub fn delete(&self, path: &str) -> reqwest::RequestBuilder {
        self.client
            .delete(format!("{}{}", self.base_url, path))
    }
}
