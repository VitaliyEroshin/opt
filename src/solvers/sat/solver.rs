use crate::p::cnf::{CNF, Literal};

pub struct Error {
    what: String
}

impl Error {
    pub fn new(s: &str) -> Error {
        Error {
            what: s.to_string()
        }
    }

    pub fn what(&self) -> &str {
        &self.what
    }
}

pub trait Solver {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error>;
}