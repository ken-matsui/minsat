///
/// DIMACS CNF file parser
///
use crate::clause::Clause;
use crate::dimacs::cnf::Cnf;
use crate::lit::Lit;

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use anyhow::{bail, Context, Result};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) trait Parse {
    fn parse(&self) -> Result<Cnf>;
}

impl Parse for Path {
    fn parse(&self) -> Result<Cnf> {
        if !self.exists() {
            bail!("The file {self:?} does not exist");
        }
        if Some(OsStr::new("cnf")) != self.extension() {
            bail!("The file extension must be `.cnf`");
        }

        let mut cnf = Cnf::default();
        // File hosts must exist in current path before this produces output
        if let Ok(lines) = read_lines(self) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines.into_iter().flatten() {
                // c start with comments
                if line.is_empty() || line.starts_with('c') {
                    continue;
                }

                let vars: Vec<&str> = line.split_whitespace().collect();

                // p cnf 5 3
                if vars[0] == "p" && vars[1] == "cnf" {
                    if vars.len() != 4 {
                        bail!("Must have 2 arguments: {line}");
                    }
                    cnf.num_vars = vars[2].parse()?;
                    cnf.num_clauses = vars[3].parse()?;
                    continue;
                }

                // 1 -5 4 0
                let mut clause = Clause::default();
                for var in vars {
                    let v = var.parse::<i32>().context(format!(
                        "var `{var}` should formatted as i32, in line: {line}"
                    ))?;
                    if v == 0 {
                        break;
                    }
                    clause.push(Lit::new(v.abs() - 1, v <= 0));
                }
                if !clause.is_empty() {
                    cnf.clauses.push(clause);
                }
            }
        }
        Ok(cnf)
    }
}
