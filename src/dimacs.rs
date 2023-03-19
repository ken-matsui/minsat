mod cnf;
mod parse;

pub(crate) use crate::dimacs::cnf::Cnf;
use crate::dimacs::parse::Parse;

use std::path::Path;

use anyhow::Result;

pub(crate) fn parse(file: &str) -> Result<Cnf> {
    let path = Path::new(file);
    path.parse()
}
