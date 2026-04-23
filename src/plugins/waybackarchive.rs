use crate::models::Subdomain;
use crate::plugins::Plugin;
use crate::errors::{DarkScoutError, Result};
use crate::browser::Browser;
use crate::utils::extract_hostname;
use async_trait::async_trait;

pub struct WaybackArchive;

#[async_trait]
impl Plugin for WaybackArchive {
    fn name(&self) -> &str {
        "WaybackArchive"
    }

    fn description(&self) -> &str {
        "OSINT source from Internet Archive's Wayback Machine"
    }

    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>> {
        let url = format!(
            "https://web.archive.org/cdx/search/cdx?url=*.{}/*&output=json&fl=original&collapse=urlkey",
            domain
        );
        
        let response = browser.get(&url, self.name()).await?;
        let text = response.text().await.map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

        // Wayback can return JSON or sometimes text depending on load/headers
        if text.trim().starts_with('[') {
            let results: Vec<Vec<String>> = serde_json::from_str(&text)
                .map_err(|e| DarkScoutError::ParseError(self.name().to_string(), e.to_string()))?;

            let subdomains: Vec<Subdomain> = results
                .into_iter()
                .skip(1)
                .filter(|row| !row.is_empty())
                .map(|row| {
                    let hostname = extract_hostname(&row[0]);
                    Subdomain { url: hostname }
                })
                .collect();
            Ok(subdomains)
        } else {
            // Parse as plain text (one URL per line)
            let subdomains: Vec<Subdomain> = text
                .lines()
                .map(|line| {
                    let hostname = extract_hostname(line);
                    Subdomain { url: hostname }
                })
                .collect();
            Ok(subdomains)
        }
    }
}
