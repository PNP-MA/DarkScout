extern crate serde;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Certificate {
    pub name_value: String,
}

#[derive(Debug, Deserialize)]
pub struct DNSEntry {
    pub hostname: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AlientVaultDNS {
    pub passive_dns: Vec<DNSEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ThreatminerResults {
    pub results: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Subdomain {
    pub url: String,
}
