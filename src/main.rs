use clap::{Parser, Subcommand};
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

pub mod extractor;

const VERSION: Option<&'static str> = option_env!("VERSION");

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
#[command(disable_version_flag(true))]
struct Cli {
    /// Timestamp in the format YYYY-MM-DD for the oldest message to find attachments for
    #[arg(long)]
    since: Option<String>,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Export all attachments from mails in given maildir
    Export {
        /// Path to the maildir to parse for attachments
        #[arg(long)]
        maildir: String,
        /// Path to output directory to write attachments to (has to exist)
        #[arg(long)]
        output_dir: String,
    },
    /// List all available things
    List {
        /// Path to the maildir to parse for attachments
        #[arg(long)]
        maildir: String,
    },
    /// Print version and exit
    Version {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List { maildir }) => {
            let extractor = extractor::Extractor::new(String::from(maildir), None);
            for attachment in extractor.list().unwrap() {
                println!("{}", attachment);
            }
        }
        Some(Commands::Export {
            maildir,
            output_dir,
        }) => {
            let extractor =
                extractor::Extractor::new(String::from(maildir), Some(String::from(output_dir)));
            match extractor.extract() {
                Err(e) => {
                    println!("Error exporting attachments:: {}", e);
                }
                _ => {}
            }
        }
        Some(Commands::Version {}) => {
            println!(
                "tool {} {} compiled on {}",
                VERSION.unwrap_or("dev"),
                CURRENT_PLATFORM,
                COMPILED_ON
            )
        }

        None => {
            println!("unknown command")
        }
    }
}
