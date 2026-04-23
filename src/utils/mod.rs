use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use console::{style, Emoji};
use crate::plugins::PluginRegistry;

pub static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍 ", "");
pub static SPARKLE: Emoji<'_, '_> = Emoji("✨ ", "");
pub static CROSS_MARK: Emoji<'_, '_> = Emoji("❌ ", "");
pub static SUCCESS_MARK: Emoji<'_, '_> = Emoji("✅ ", "");

// Remove beginning protocols from the target url
pub fn sanitize_target_url_string(raw_target: String) -> String {
    raw_target
        .replace("www.", "")
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "")
        .replace("https://www.", "")
}

pub fn create_progress_bar(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(message.to_string());
    pb
}

pub fn extract_hostname(url: &str) -> String {
    url.replace("https://", "")
       .replace("http://", "")
       .split('/')
       .next()
       .unwrap_or("")
       .to_string()
}

pub fn print_step(msg: &str) {
    println!("{} {}", style("[darkscout]").cyan().bold(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {} {} {}", style("[darkscout]").red().bold(), CROSS_MARK, style("error:").red(), msg);
}

pub fn print_success(msg: &str) {
    println!("{} {} {} {}", style("[darkscout]").green().bold(), SUCCESS_MARK, style("success:").green(), msg);
}

pub fn print_opening() {
    let logo = r#"
    ____             __   _____                     __ 
   / __ \____ ______/ /__/ ___/_________  __  __/ /_
  / / / / __ `/ ___/ //_/\__ \/ ___/ __ \/ / / / __/
 / /_/ / /_/ / /  / ,<  ___/ / /__/ /_/ / /_/ / /_  
/_____/\__,_/_/  /_/|_|/____/\___/\____/\__,_/\__/  
    "#;
    println!("{}", style(logo).cyan().bold());
    println!("  {}", style("Modern Subdomain Enumeration Framework").dim());
    println!();
    println!("  {} {}", style("Repository:").dim(), style("https://github.com/DarkSuite/DarkScout").blue().underlined());
    println!("  {} {}", style("Version:   ").dim(), style(env!("CARGO_PKG_VERSION")).yellow());
    println!();
}

pub fn list_all_plugins(registry: &PluginRegistry) {
    println!("{}", style("Available Plugins:").bold().underlined());
    for plugin in registry.get_all() {
        let status = if plugin.enabled() { 
            style("(enabled)").green().dim() 
        } else { 
            style("(disabled)").red().dim() 
        };
        println!("  • {:<15} - {} {}", style(plugin.name()).cyan(), plugin.description(), status);
    }
    println!();
}
