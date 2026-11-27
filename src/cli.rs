use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]

pub struct CliArgs {
    #[arg(long)]
    pub machines: PathBuf,

    #[arg(long)]
    pub orders: PathBuf,

    #[arg(long)]
    pub operations: PathBuf,

    #[arg(long)]
    pub machines_capability: PathBuf,

    #[arg(long)]
    pub companies:PathBuf
}

pub fn cli() -> CliArgs {
    CliArgs::parse()
}
