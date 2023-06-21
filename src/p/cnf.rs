use std::{collections::HashSet, hash::Hash};
use std::fmt::{Debug};

#[derive(Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Literal {
    pub var: usize,
    pub sign: bool,
}

pub struct CNF {
    clauses: HashSet<Vec<Literal>>,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self.sign { "-" } else { "" })?;
        write!(f, "{}", self.var)
    }
}

impl Literal {
    pub fn new(s: &str) -> Result<Literal, std::num::ParseIntError> {
        let i = s.parse::<i32>()?;
        Ok(Literal {
            var: i.abs() as usize,
            sign: i < 0,
        })
    }

    pub fn neg(&self) -> Literal {
        return Literal {
            var: self.var,
            sign: !self.sign,
        };
    }
}

fn print_vec_with_separator(literals: &Vec<Literal>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut iter = literals.iter();

    match iter.next() {
        None => {}
        Some(value) => {
            write!(f, "{:?}", value)?
        }
    }

    for value in iter {
        write!(f, " {:?}", value)?
    }

    Ok(())
}

impl std::fmt::Display for CNF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n", self.clauses.len())?;
        let mut iter = self.clauses.iter();

        match iter.next() {
            None => {}
            Some(clause) => {
                print_vec_with_separator(clause, f)?;
            }
        }

        for clause in iter {
            write!(f, "\n")?;
            print_vec_with_separator(clause, f)?;
        }
        Ok(())
    }
}

impl CNF {
    pub fn new() -> CNF {
        CNF {
            clauses: HashSet::new(),
        }
    }

    pub fn add_clause(&mut self, mut clause: Vec<Literal>) {
        clause.sort();
        clause.dedup();
        self.clauses.insert(clause);
    }

    pub fn get_clauses(&mut self) -> &mut HashSet<Vec<Literal>> {
        return &mut self.clauses;
    }

    pub fn get_any_literal(&self) -> Option<Literal> {
        for clause in self.clauses.iter() {
            if !clause.is_empty() {
                return Some(clause[0]);
            }
        }
        return None;
    }

    pub fn has_empty_clause(&self) -> bool {
        for clause in self.clauses.iter() {
            if clause.is_empty() {
                return true;
            }
        }
        return false;
    }

    pub fn remove_clauses_with_literal(&mut self, l: Literal) {
        let mut removed_clauses = Vec::new();
        for clause in self.clauses.iter() {
            if clause.contains(&l) || clause.contains(&l.neg()) {
                removed_clauses.push(clause.clone());
            }
        }
        for clause in removed_clauses {
            self.clauses.remove(&clause);
        }
    }

    pub fn remove_clauses(&mut self, clauses: &HashSet<Vec<Literal>>) {
        for clause in clauses.iter() {
            self.clauses.remove(clause);
        }
    }

    pub fn add_clauses(&mut self, clauses: &HashSet<Vec<Literal>>) {
        for clause in clauses.iter() {
            self.clauses.insert(clause.clone());
        }
    }
}
