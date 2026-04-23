use crate::models::{AlientVaultDNS, Subdomain};
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

pub struct AlienVault;

#[async_trait]
impl Plugin for AlienVault {
    fn name(&self) -> &str {
        "AlienVault"
    }

    fn description(&self) -> &str {
        "OSINT source from AlienVault OTX (Requires ALIENVAULT_API_KEY in .env for best results)"
    }

    fn enabled(&self) -> bool {
        // We enable it by default now, but it might fail without a key
        true 
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!(
            "https://otx.alienvault.com/api/v1/indicators/domain/{}/passive_dns",
            domain
        );
        
        let mut headers = HeaderMap::new();
        if let Ok(key) = env::var("ALIENVAULT_API_KEY") {
            if let Ok(val) = HeaderValue::from_str(&key) {
                headers.insert("X-OTX-API-KEY", val);
            }
        }

        let response = browser.get_with_headers(&url, self.name(), headers).await?;

        if response.status() == 403 || response.status() == 401 {
            return Err(DarkScoutError::AuthError(
                self.name().to_string(), 
                "Authentication required for AlienVault. Set ALIENVAULT_API_KEY in .env".to_string()
            ));
        }

        let results: AlientVaultDNS = response
            .json()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = results
            .passive_dns
            .into_iter()
            .filter(|sub| sub.hostname.is_some())
            .map(|sub| Subdomain {
                url: sub.hostname.unwrap(),
            })
            .collect();

        Ok(subdomains)
    }
}
