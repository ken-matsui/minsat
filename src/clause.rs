use crate::lit::Lit;

use std::{fmt, ops};

#[derive(Default)]
pub(crate) struct Clause(pub(crate) Vec<Lit>);

impl ops::Deref for Clause {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Clause {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .map(|lit| lit.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl Clause {
    pub(crate) fn push(&mut self, value: Lit) {
        self.0.push(value);
    }
}
