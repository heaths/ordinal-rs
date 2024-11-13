// Copyright 2020 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

mod docgen;

use clap::Parser;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    Docgen(docgen::Command),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config = Config::default();

    match args.cmd {
        Command::Docgen(cmd) => cmd.run(&config),
    }
}

struct Config {
    repo_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        Self {
            repo_dir: manifest_dir
                .parent()
                .expect("xtask parent directory")
                .to_path_buf(),
        }
    }
}
