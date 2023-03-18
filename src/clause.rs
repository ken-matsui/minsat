use crate::lit::Lit;

use std::{fmt, ops};

#[derive(Default)]
pub(crate) struct Clause(pub(crate) Vec<Lit>);

impl<'a> ops::Deref for Clause {
    type Target = Vec<Lit>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Clause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.iter().fold(Ok(()), |result, lit| {
            result.and_then(|_| write!(f, "{lit:?} "))
        })?;
        write!(f, "0")
    }
}

impl Clause {
    pub(crate) fn push(&mut self, value: Lit) {
        self.0.push(value);
    }
}
