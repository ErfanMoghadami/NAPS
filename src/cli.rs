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
}

pub fn cli() -> CliArgs {
    CliArgs::parse()
}
