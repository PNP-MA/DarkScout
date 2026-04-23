use async_trait::async_trait;
use crate::models::Subdomain;
use crate::utils::create_progress_bar;
use crate::errors::Result;
use crate::browser::Browser;
use std::time::Duration;
use indicatif::MultiProgress;
use console::style;
use std::collections::BTreeMap;

pub mod alienvault;
pub mod anubis;
pub mod crtsh;
pub mod hackertarget;
pub mod threatminer;
pub mod waybackarchive;
pub mod bevigil;

use alienvault::AlienVault;
use anubis::Anubis;
use crtsh::Crtsh;
use hackertarget::HackerTarget;
use threatminer::ThreatMiner;
use waybackarchive::WaybackArchive;
use bevigil::BeVigil;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn enabled(&self) -> bool { true }
    
    async fn fetch(&self, domain: &str, browser: &Browser) -> Result<Vec<Subdomain>>;

    async fn run(&self, domain: &str, mp: &MultiProgress, browser: &Browser) -> Result<Vec<Subdomain>> {
        let pb = mp.add(create_progress_bar(&format!("Scraping {}...", self.name())));
        
        // Emulate the delay found in original code
        tokio::time::sleep(Duration::from_secs(2)).await;

        let result = self.fetch(domain, browser).await;

        match result {
            Ok(subdomains) => {
                pb.finish_with_message(format!("{}: {} subdomains found", style(self.name()).cyan(), subdomains.len()));
                Ok(subdomains)
            }
            Err(e) => {
                pb.finish_with_message(format!("{}: {}", style(self.name()).red(), style("failed").red()));
                Err(e)
            }
        }
    }
}

pub struct PluginRegistry {
    plugins: BTreeMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            plugins: BTreeMap::new(),
        };
        registry.register(Box::new(AlienVault));
        registry.register(Box::new(Anubis));
        registry.register(Box::new(Crtsh));
        registry.register(Box::new(HackerTarget));
        registry.register(Box::new(ThreatMiner));
        registry.register(Box::new(WaybackArchive));
        registry.register(Box::new(BeVigil));
        registry
    }

    fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.insert(plugin.name().to_lowercase(), plugin);
    }

    pub fn get_all(&self) -> Vec<&dyn Plugin> {
        self.plugins.values().map(|p| p.as_ref()).collect()
    }

    pub fn get_enabled(&self) -> Vec<&dyn Plugin> {
        self.plugins.values()
            .map(|p| p.as_ref())
            .filter(|p| p.enabled())
            .collect()
    }

    pub fn get_by_names(&self, names: &[String]) -> Vec<&dyn Plugin> {
        let mut selected = BTreeMap::new();
        for name in names {
            let lower_name = name.to_lowercase();
            if let Some(plugin) = self.plugins.get(&lower_name) {
                selected.insert(lower_name, plugin.as_ref());
            }
        }
        selected.into_values().collect()
    }
}
