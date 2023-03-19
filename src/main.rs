#![feature(let_chains)]

mod clause;
mod dimacs;
mod lit;
mod solve;

use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// the path to the DIMACS CNF file
    filepath: String,

    /// Load cnf from the examples directory: examples/cnf/
    #[clap(short, long)]
    example: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cnf = if cli.example {
        dimacs::parse(&format!("examples/cnf/{}", cli.filepath))?
    } else {
        dimacs::parse(&cli.filepath)?
    };
    println!("{cnf}");

    Ok(())
}
