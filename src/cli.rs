use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "photo-explorer")]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "A tool for finding information about a directory of photos")]
pub struct Cli {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Displays the averages for metadata on the photos
    Average(CommonArgs),
}

#[derive(Debug, Args)]
pub struct CommonArgs {
    /// The directory to search for photos
    #[arg(default_value = ".")]
    pub directory: String,
    /// Provides more information when running
    #[arg(short, long)]
    pub verbose: bool,
}
