use crate::models::{Certificate, Subdomain};
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;
use std::time::Duration;

pub struct Crtsh;

#[async_trait]
impl Plugin for Crtsh {
    fn name(&self) -> &str {
        "CrtSH"
    }

    fn description(&self) -> &str {
        "OSINT source from crt.sh certificate logs (Includes auto-retry for 502 errors)"
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!("https://crt.sh/?q={}&output=json", domain);
        
        let mut response = browser.get(&url, self.name()).await?;
        
        // Simple retry for 502/503 errors which are common on crt.sh
        if response.status().is_server_error() {
            tokio::time::sleep(Duration::from_secs(2)).await;
            response = browser.get(&url, self.name()).await?;
        }

        let certificates: Vec<Certificate> = response
            .json()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = certificates
            .into_iter()
            .flat_map(|cert| {
                cert.name_value
                    .split('\n')
                    .map(|dstr| dstr.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|dstr| dstr != domain)
            .filter(|dstr| !dstr.contains('*'))
            .map(|dstr| Subdomain { url: dstr })
            .collect();

        Ok(subdomains)
    }
}
