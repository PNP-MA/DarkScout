use crate::models::Subdomain;
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;

pub struct Anubis;

#[async_trait]
impl Plugin for Anubis {
    fn name(&self) -> &str {
        "Anubis"
    }

    fn description(&self) -> &str {
        "OSINT source from Anubis (jonlu.ca)"
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!("https://jonlu.ca/anubis/subdomains/{}", domain);
        
        let response = browser.get(&url, self.name()).await?;

        let results: Vec<String> = response
            .json()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = results
            .into_iter()
            .map(|sub| Subdomain { url: sub })
            .collect();

        Ok(subdomains)
    }
}
