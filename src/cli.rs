use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The directory to search for photos
    #[arg()]
    pub directory: String,

    /// Provides more information when running
    #[arg(short, long)]
    pub verbose: bool,
}
