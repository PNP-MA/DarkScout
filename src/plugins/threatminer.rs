use crate::models::{Subdomain, ThreatminerResults};
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use async_trait::async_trait;

pub struct ThreatMiner;

#[async_trait]
impl Plugin for ThreatMiner {
    fn name(&self) -> &str {
        "ThreatMiner"
    }

    fn description(&self) -> &str {
        "OSINT source from ThreatMiner API"
    }

    fn enabled(&self) -> bool {
        false // API currently dead
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!("https://api.threatminer.org/v2/domain.php?q={}&rt=5", domain);
        
        let response = browser.get(&url, self.name()).await?;

        let results: ThreatminerResults = response
            .json()
            .await
            .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        let subdomains: Vec<Subdomain> = results
            .results
            .into_iter()
            .map(|sub| Subdomain { url: sub })
            .collect();

        Ok(subdomains)
    }
}
