use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[command(
    author, 
    version, 
    about,
    arg_required_else_help = true,
    help_template = "{before-help}{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
pub struct Arguments {
    #[arg(short, long, env("TARGET_URL"), help = "Target URL or domain (e.g. hackthissite.org)")]
    pub target_url: Option<String>,

    #[arg(short, long, env("OUTPUT_FILE"), help = "File to save the discovered subdomains")]
    pub output_file: Option<String>,

    #[arg(short, long, value_delimiter = ',', help = "List of plugins to run (e.g. Alienvault,Anubis). Defaults to all")]
    pub plugins: Option<Vec<String>>,

    #[arg(short, long, help = "List all available plugins and their status")]
    pub list: bool,

    #[arg(short, long, value_delimiter = ',', help = "Path to wordlist file, multiple files, or directory for brute forcing")]
    pub wordlist: Option<Vec<String>>,

    #[arg(short, long, default_value = "100", help = "Number of concurrent DNS lookups for brute forcing")]
    pub concurrency: usize,
}

impl Arguments {
    pub fn from_env_and_args() -> Self {
        dotenv().ok();
        Self::parse()
    }
}
