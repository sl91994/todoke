mod cli;
mod commands;
mod model;
mod output;
mod store;
mod template;

use crate::cli::Args;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    commands::dispatch(args)
}
