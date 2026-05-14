use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "go2rust-cli")]
#[command(about = "Convert Go source files into Rust skeletons")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Convert {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(long)]
        check: bool,
    },
}
