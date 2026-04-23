use crate::models::Subdomain;
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;

pub struct HackerTarget;

#[async_trait]
impl Plugin for HackerTarget {
    fn name(&self) -> &str {
        "HackerTarget"
    }

    fn description(&self) -> &str {
        "OSINT source from HackerTarget API"
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!("https://api.hackertarget.com/hostsearch/?q={}", domain);
        
        let response = browser.get(&url, self.name()).await?;

        let text = response
            .text()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = text
            .lines()
            .map(|line| {
                let url = line.split(',').next().unwrap_or("").to_string();
                Subdomain { url }
            })
            .collect();

        Ok(subdomains)
    }
}
