mod collector;
mod models;
mod security;
mod utils;

use anyhow::Result;
use clap::Parser;
use security::permissions::SecurityContext;
use collector::unified::UnifiedLogCollector;
use tracing::{info, error};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "1000")]
    max_entries: usize,

    #[arg(short, long, default_value = "json")]
    format: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let security_context = SecurityContext::new()?;
    if !security_context.has_sufficient_permissions()? {
        error!("Insufficient permissions to access logs");
        std::process::exit(1);
    }

    let unified_collector = UnifiedLogCollector::new(
        "subsystem == \"com.apple.security\"",
        args.max_entries,
    );

    match unified_collector.collect_logs().await {
        Ok(logs) => {
            match args.format.as_str() {
                "json" => println!("{}", serde_json::to_string_pretty(&logs)?),
                "text" => {
                    for log in logs {
                        println!("[{}] {}: {}", log.timestamp, log.subsystem, log.message);
                    }
                }
                _ => error!("Unsupported output format"),
            }
        }
        Err(e) => error!("Failed to collect logs: {}", e),
    }

    Ok(())
}
