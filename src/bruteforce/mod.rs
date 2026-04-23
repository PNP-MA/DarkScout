use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use crate::models::Subdomain;
use crate::utils::create_progress_bar;
use std::sync::Arc;
use tokio::sync::Semaphore;
use futures::future::join_all;
use console::style;
use indicatif::MultiProgress;

pub struct BruteForceEngine {
    resolver: TokioAsyncResolver,
    concurrency: usize,
}

impl BruteForceEngine {
    pub fn new(concurrency: usize) -> Self {
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        ).unwrap_or_else(|_| {
            TokioAsyncResolver::tokio(
                ResolverConfig::google(),
                ResolverOpts::default(),
            ).unwrap()
        });
        
        Self {
            resolver,
            concurrency,
        }
    }

    pub async fn run(&self, domain: &str, wordlist: Vec<String>, mp: &MultiProgress) -> Vec<Subdomain> {
        let pb = mp.add(create_progress_bar(&format!("Brute forcing {} words...", wordlist.len())));
        let semaphore = Arc::new(Semaphore::new(self.concurrency));
        let resolver = Arc::new(self.resolver.clone());
        
        let mut futures = Vec::new();

        for word in wordlist {
            let sem = semaphore.clone();
            let res = resolver.clone();
            let full_domain = format!("{}.{}", word, domain);
            
            futures.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.ok();
                match res.lookup_ip(&full_domain).await {
                    Ok(_) => Some(Subdomain { url: full_domain }),
                    Err(_) => None,
                }
            }));
        }

        let mut results = Vec::new();
        for f in join_all(futures).await {
            if let Ok(Some(sub)) = f {
                results.push(sub);
            }
            pb.tick();
        }

        pb.finish_with_message(format!("{}: {} subdomains found", style("BruteForce").cyan(), results.len()));
        results
    }
}
