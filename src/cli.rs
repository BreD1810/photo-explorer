use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "photo-explorer")]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "A tool for finding information about a directory of photos")]
pub struct Args {
    /// Calculate the average values of metadata
    #[arg(short, long)]
    pub average: bool,
    /// The directory to search for photos
    #[arg(default_value = ".")]
    pub directory: String,
    /// Provides more information when running
    #[arg(short, long)]
    pub verbose: bool,
}
