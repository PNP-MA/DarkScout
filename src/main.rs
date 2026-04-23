use itertools::Itertools;
use futures::future::join_all;
use indicatif::MultiProgress;
use console::style;
use std::collections::HashSet;

mod plugins;
mod models;
mod utils;
mod io;
mod errors;
mod browser;
mod args;
mod bruteforce;

use plugins::PluginRegistry;
use args::Arguments;
use bruteforce::BruteForceEngine;
use utils::{print_step, print_error, print_success, print_opening, list_all_plugins, LOOKING_GLASS, SPARKLE};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Arguments::from_env_and_args();
    let registry = PluginRegistry::new();

    if args.list {
        print_opening();
        list_all_plugins(&registry);
        return Ok(());
    }

    let raw_target = match args.target_url {
        Some(target) => target,
        None => {
            print_opening();
            return Ok(());
        }
    };

    // Print opening only for actual execution
    print_opening();

    let start = std::time::Instant::now();
    let cleaned_target = utils::sanitize_target_url_string(raw_target);  

    print_step(&format!("{} Target: {}", LOOKING_GLASS, style(&cleaned_target).bold().yellow()));
    println!();

    let mut all_found_subdomains = Vec::new();
    let mp = MultiProgress::new();

    // --- PHASE 1: OSINT Plugins ---
    // Logic: 
    // 1. If plugins are explicitly requested (-p), run them.
    // 2. If no plugins are requested AND no wordlist is provided, run enabled plugins.
    // 3. If wordlist is provided (-w) AND no plugins are requested (-p), skip OSINT.
    let engines = if let Some(plugin_names) = &args.plugins {
        registry.get_by_names(plugin_names)
    } else if args.wordlist.is_none() {
        registry.get_enabled()
    } else {
        Vec::new() // Skip OSINT if wordlist provided without -p
    };

    if !engines.is_empty() {
        let browser = browser::Browser::new();
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(3)); 
        
        let mut ran_plugins = HashSet::new();
        let mut futures = Vec::new();

        for engine in engines {
            let name = engine.name().to_string();
            if ran_plugins.contains(&name) { continue; }
            ran_plugins.insert(name);

            let sem = semaphore.clone();
            let target = cleaned_target.clone();
            let mp_ref = &mp;
            let br = &browser;
            
            futures.push(async move {
                let _permit = sem.acquire().await.ok();
                engine.run(&target, mp_ref, br).await
            });
        }
        
        let results = join_all(futures).await;
        for res in results {
            if let Ok(subs) = res {
                all_found_subdomains.extend(subs);
            }
        }
    }

    // --- PHASE 2: Brute Force ---
    if let Some(wordlist_paths) = &args.wordlist {
        let words = io::read_wordlists(wordlist_paths)?;
        if !words.is_empty() {
            let brute_engine = BruteForceEngine::new(args.concurrency);
            let brute_results = brute_engine.run(&cleaned_target, words, &mp).await;
            all_found_subdomains.extend(brute_results);
        } else {
            print_error("Wordlist is empty or no valid files found.");
        }
    }

    // Deduplicate and normalize
    let subdomains: Vec<_> = all_found_subdomains
        .into_iter()
        .map(|mut s| {
            s.url = s.url.trim().to_lowercase();
            s
        })
        .filter(|s| !s.url.is_empty() && s.url.contains(&cleaned_target))
        .unique_by(|s| s.url.clone())
        .sorted_by(|a, b| a.url.cmp(&b.url))
        .collect();

    let duration = start.elapsed();
    let total = subdomains.len();

    println!();
    print_step(&format!("{} Found {} unique subdomains", SPARKLE, style(total).bold().green()));
    println!();

    if total > 0 {
        for sub in &subdomains {
            println!("  {} {}", style("→").dim(), sub.url);
        }
    }

    if let Some(output_file) = args.output_file {
        if let Err(e) = io::create_output_dir() {
            print_error(&format!("Failed to create output directory: {}", e));
        } else if let Err(e) = io::create_output_file(&output_file, &subdomains) {
            print_error(&format!("Failed to write output file: {}", e));
        } else {
            println!();
            print_success(&format!("Results saved to {}", style(format!("output/{}", output_file)).underlined()));
        }
    }

    println!();
    print_step(&format!("Finished in {}", style(format!("{:?}", duration)).cyan()));
    println!();

    Ok(())
}
