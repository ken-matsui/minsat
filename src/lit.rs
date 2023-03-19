use std::fmt;

use eh::Eh;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct Lit {
    x: i32,
}

impl Lit {
    pub(crate) fn new(x: i32, is_neg: bool) -> Self {
        assert!(x >= 0);
        Lit {
            x: 2 * x + is_neg as i32,
        }
    }

    #[inline]
    pub(crate) fn is_pos(&self) -> bool {
        !self.is_neg()
    }
    #[inline]
    pub(crate) fn is_neg(&self) -> bool {
        (self.x & 1).eh()
    }

    /// Negate this literal
    #[inline]
    pub(crate) fn neg(&mut self) -> &Self {
        self.x ^= 1;
        self
    }

    #[inline]
    pub(crate) fn var(&self) -> i32 {
        self.x >> 1
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_neg() { "-" } else { "" },
            self.var() + 1
        )
    }
}
