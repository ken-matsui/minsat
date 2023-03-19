use crate::clause::Clause;

use std::fmt;

#[derive(Default)]
pub(crate) struct Cnf {
    pub(crate) num_vars: usize,
    pub(crate) num_clauses: usize,
    pub(crate) clauses: Vec<Clause>,
}

impl fmt::Display for Cnf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "p cnf {} {}", self.num_vars, self.num_clauses)?;
        self.clauses.iter().fold(Ok(()), |result, clause| {
            result.and_then(|_| writeln!(f, "{clause} 0"))
        })
    }
}
