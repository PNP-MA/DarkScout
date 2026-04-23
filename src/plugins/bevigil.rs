use crate::models::Subdomain;
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;
use serde::Deserialize;

pub struct BeVigil;

#[derive(Deserialize)]
struct BeVigilResponse {
    subdomains: Vec<String>,
}

#[async_trait]
impl Plugin for BeVigil {
    fn name(&self) -> &str {
        "BeVigil"
    }

    fn description(&self) -> &str {
        "OSINT source from BeVigil OSINT API (Requires BEVIGIL_API_KEY in .env)"
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!("https://osint.bevigil.com/api/{}/subdomains/", domain);
        
        let mut headers = HeaderMap::new();
        if let Ok(key) = env::var("BEVIGIL_API_KEY") {
            if let Ok(val) = HeaderValue::from_str(&key) {
                headers.insert("X-Access-Token", val);
            }
        } else {
            // BeVigil requires an API key
            return Err(DarkScoutError::AuthError(
                self.name().to_string(), 
                "API Key is required. Set BEVIGIL_API_KEY in .env".to_string()
            ));
        }

        let response = browser.get_with_headers(&url, self.name(), headers).await?;

        if response.status() == 403 || response.status() == 401 {
            return Err(DarkScoutError::AuthError(
                self.name().to_string(), 
                "Invalid API Key or credits exhausted.".to_string()
            ));
        }

        let results: BeVigilResponse = response
            .json()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = results
            .subdomains
            .into_iter()
            .map(|sub| Subdomain { url: sub })
            .collect();

        Ok(subdomains)
    }
}
