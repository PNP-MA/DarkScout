use reqwest::{Client, Response, header::{HeaderMap}};
use crate::errors::{DarkScoutError, Result};
use std::time::Duration;

pub struct Browser {
    client: Client,
    user_agent: String,
}

impl Browser {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60)) // Wayback and crt.sh can be very slow
                .build()
                .unwrap_or_else(|_| Client::new()),
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36".to_string(),
        }
    }

    pub async fn get_with_headers(&self, url: &str, plugin_name: &str, headers: HeaderMap) -> Result<Response> {
        let mut req = self.client.get(url).header("User-Agent", &self.user_agent);
        
        for (key, value) in headers.iter() {
            req = req.header(key.clone(), value.clone());
        }

        req.send()
            .await
            .map_err(|e| DarkScoutError::NetworkError(plugin_name.to_string(), e))
    }

    pub async fn get(&self, url: &str, plugin_name: &str) -> Result<Response> {
        self.get_with_headers(url, plugin_name, HeaderMap::new()).await
    }
}
