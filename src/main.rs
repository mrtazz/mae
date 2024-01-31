use chrono::prelude::*;
use clap::{Parser, Subcommand};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

pub mod extractor;
pub mod sanitize;

const VERSION_FROM_CARGO: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const VERSION: Option<&'static str> = option_env!("VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
#[command(disable_version_flag(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Extract all attachments from mails in given maildir
    Extract {
        /// Path to the maildir to parse for attachments
        #[arg(long)]
        maildir: String,
        /// Path to output directory to write attachments to (has to exist)
        #[arg(long)]
        output_dir: String,
        /// Timestamp in the format YYYY-MM-DD for the oldest message to find attachments for
        #[arg(long)]
        since: Option<String>,
        /// Comma separated list of file suffixes to export (e.g. .pdf,.doc)
        #[arg(long)]
        suffixes: Option<String>,
    },
    /// List all available attachments with the given (or default) suffixes
    List {
        /// Path to the maildir to parse for attachments
        #[arg(long)]
        maildir: String,
        /// Timestamp in the format YYYY-MM-DD for the oldest message to find attachments for
        #[arg(long)]
        since: Option<String>,
        /// Comma separated list of file suffixes to export (e.g. .pdf,.doc)
        #[arg(long)]
        suffixes: Option<String>,
    },
    /// Print version and exit
    Version {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List {
            maildir,
            since,
            suffixes,
        }) => {
            let mut extractor = extractor::Extractor::new(String::from(maildir), None);

            if let Some(since) = since {
                match NaiveDate::parse_from_str(&since, "%Y-%m-%d") {
                    Ok(parsed_since) => {
                        extractor = extractor.since(parsed_since);
                    }
                    Err(e) => {
                        println!(
                            "Not able to parse provided --since={} into date: {}",
                            since, e
                        );
                        return;
                    }
                }
            }
            if let Some(suffixes) = suffixes {
                extractor =
                    extractor.with_suffixes(suffixes.split(",").map(|s| s.to_string()).collect());
            }

            for attachment in extractor.list().unwrap() {
                println!("{}", attachment);
            }
        }
        Some(Commands::Extract {
            maildir,
            output_dir,
            since,
            suffixes,
        }) => {
            let mut extractor =
                extractor::Extractor::new(String::from(maildir), Some(String::from(output_dir)));

            if let Some(since) = since {
                match NaiveDate::parse_from_str(&since, "%Y-%m-%d") {
                    Ok(parsed_since) => {
                        extractor = extractor.since(parsed_since);
                    }
                    Err(e) => {
                        println!(
                            "Not able to parse provided --since={} into date: {}",
                            since, e
                        );
                        return;
                    }
                }
            }
            if let Some(suffixes) = suffixes {
                extractor =
                    extractor.with_suffixes(suffixes.split(",").map(|s| s.to_string()).collect());
            }
            match extractor.extract() {
                Err(e) => {
                    println!("Error exporting attachments:: {}", e);
                }
                _ => {}
            }
        }
        Some(Commands::Version {}) => {
            println!(
                "mae {} {} compiled on {}",
                VERSION.unwrap_or(VERSION_FROM_CARGO.unwrap_or("dev")),
                CURRENT_PLATFORM,
                COMPILED_ON
            )
        }

        None => {
            println!("unknown command")
        }
    }
}
