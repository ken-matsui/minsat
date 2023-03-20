#![feature(let_chains)]

mod dimacs;
mod solver;
mod types;

use crate::solver::Status;

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
    let result = solver::solve(cnf);
    println!("s {result}");
    if let Status::Sat { assigns } = result {
        for (index, var) in assigns.into_iter().enumerate() {
            if var {
                print!("{} ", index + 1);
            } else {
                print!("-{} ", index + 1);
            }
        }
        println!("0");
    }

    Ok(())
}
