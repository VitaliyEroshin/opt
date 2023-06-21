use std::{collections::HashSet, hash::Hash};
use std::fmt::{Debug};

#[derive(Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub struct Literal {
    pub var: usize,
    pub sign: bool,
}

#[derive(Clone)]
pub struct CNF {
    clauses: HashSet<Vec<Literal>>,
    variables: usize,
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", if self.sign { "-" } else { "" })?;
        write!(f, "{}", self.var)
    }
}

impl Literal {
    pub fn from_int(i: i32) -> Literal {
        Literal {
            var: i.abs() as usize,
            sign: i < 0
        }
    }

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

    pub fn get_var(&self) -> usize {
        return self.var;
    }

    pub fn is_negative(&self) -> bool {
        return self.sign;
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
            variables: 0,
        }
    }

    pub fn var_count(&self) -> usize {
        return self.variables;
    }

    pub fn add_clause(&mut self, mut clause: Vec<Literal>) {
        clause.sort();
        clause.dedup();
        for v in clause.iter() {
            self.variables = std::cmp::max(self.variables, v.get_var())
        }
        self.clauses.insert(clause);
    }

    pub fn get_clauses(&mut self) -> &mut HashSet<Vec<Literal>> {
        &mut self.clauses
    }

    pub fn clauses(&self) -> &HashSet<Vec<Literal>> {
        &self.clauses
    }

    pub fn eval(&self, eval_vec: Vec<Literal>) -> bool {
        let eval_set = HashSet::<Literal>::from_iter(eval_vec.iter().cloned());

        for clause in self.clauses.iter() {
            if !Self::eval_clause(clause, &eval_set) {
                return false;
            }
        }

        return true;
    }

    fn eval_clause(clause: &Vec<Literal>, eval_set: &HashSet<Literal>) -> bool {
        for l in clause.iter() {
            if eval_set.contains(l) {
                return true;
            }
        }
        return false;
    }
}
