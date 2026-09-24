use std::num::NonZeroUsize;

use clap::{Parser, Subcommand};

mod commands;
mod path;

#[derive(Parser, Debug)]
#[command(name = "bucket")]
#[command(about = "A stupidly simple bucket list.")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new .bucketlist
    Init,

    /// Add something to your bucket
    Add { item: String },

    /// List your bucket
    List,

    /// Complete a bucket item
    Kick { item: NonZeroUsize },

    /// Remove an item from your bucket
    Rm { item: NonZeroUsize },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init()?,
        Commands::Add { item } => commands::add(&item)?,
        Commands::Kick { item } => commands::kick(item)?,
        Commands::Rm { item } => commands::remove(item)?,
        Commands::List => commands::list()?,
    }

    Ok(())
}
