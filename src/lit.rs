use std::fmt;

use eh::Eh;

pub(crate) struct Lit {
    x: i32,
}

impl Lit {
    pub(crate) fn new(x: i32, neg: bool) -> Self {
        assert!(x >= 0);
        Lit {
            x: 2 * x + neg as i32,
        }
    }

    #[inline]
    pub(crate) fn pos(&self) -> bool {
        !self.neg()
    }
    #[inline]
    pub(crate) fn neg(&self) -> bool {
        (self.x & 1).eh()
    }

    #[inline]
    pub(crate) fn var(&self) -> i32 {
        self.x >> 1
    }
}

impl fmt::Debug for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.neg() { "-" } else { "" }, self.var() + 1)
    }
}
