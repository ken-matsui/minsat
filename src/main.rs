#![feature(let_chains)]

mod clause;
mod dimacs;
mod lit;

use anyhow::Result;

enum Res {
    Sat,
    Unsat,
    Normal,
}

fn main() -> Result<()> {
    let cnf = dimacs::parse("examples/cnf/3SAT.cnf")?;
    println!("{cnf:#?}");

    Ok(())
}
