use clap::{Args, Parser, Subcommand};

/// Parse from the CLI
#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Download(DownloadTask),
}

#[derive(Subcommand)]
pub enum DownloadTask {
    /// Alert started download task
    Started(StartedDownloadTask),
    /// Alert finished download task
    Finished(FinishedDownloadTask),
}

#[derive(Args)]
pub struct StartedDownloadTask {
    /// Path of file started downloading.
    #[clap(short, long)]
    pub file_path: String,
}

#[derive(Args)]
pub struct FinishedDownloadTask {
    /// Passed in from caller whether file was moved.
    #[clap(short, long, action)]
    pub has_moved: bool,
    /// Path of file whether moved or not.
    #[clap(short, long)]
    pub file_path: String,
}