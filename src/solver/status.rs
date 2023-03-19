use std::fmt;

pub(crate) enum Status {
    Sat { assigns: Vec<bool> },
    Unsat,
    Unknown,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Status::Sat { .. } => "SAT",
                Status::Unsat => "UNSAT",
                Status::Unknown => "UNKNOWN",
            }
        )
    }
}
