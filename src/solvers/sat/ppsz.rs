use std::{mem::swap};

use rand::seq::SliceRandom;
use rand::{distributions::Uniform, Rng};

pub use super::solver::{Solver, Error};
use crate::p::cnf::{CNF, Literal};

pub struct PPSZ {
    max_clauses: usize,
    max_resolve_iterations: usize,
    max_search_iterations: usize,
    max_clause_size: usize,
    bounded_resolve_iterations: usize,
}

impl Solver for PPSZ {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error> {
        self.solve_ppsz(cnf)
    }
}

impl PPSZ {
    pub fn new() -> PPSZ {
        PPSZ {
            max_clauses: 500,
            max_resolve_iterations: 10,
            max_search_iterations: 100,
            max_clause_size: 3,
            bounded_resolve_iterations: 2,
        }
    }

    pub fn set_max_clauses(&mut self, value: usize) {
        self.max_clauses = value;
    }

    pub fn set_max_resolve_iterations(&mut self, value: usize) {
        self.max_resolve_iterations = value;
    }

    pub fn set_max_search_iterations(&mut self, value: usize) {
        self.max_search_iterations = value;
    }

    pub fn set_max_clause_size(&mut self, value: usize) {
        self.max_clause_size = value;
    }

    pub fn set_bounded_resolve_iterations(&mut self, value: usize) {
        self.bounded_resolve_iterations = value;
    }

    pub fn solve_ppsz(&self, mut cnf: CNF) -> Result<Vec<Literal>, Error> {
        let max_clauses = self.max_clauses;

        for _it in 0..self.max_resolve_iterations {
            if cnf.clauses().len() < self.max_clauses {
                Self::bounded_resolve(&self, &mut cnf, self.max_clause_size, max_clauses);
            }

            match Self::search(&mut cnf, self.max_search_iterations) {
                None => {},
                Some(eval_set) => {
                    return Ok(eval_set);
                }
            }
        }

        return Err(Error::new("Unsatisfiable"));
    }

    fn bounded_resolve(&self, g: &mut CNF, s: usize, max_clauses: usize) {
        let mut clauses: Vec<Vec<Literal>> = g.clauses().iter().cloned().collect();
        let (mut l, mut r) = (0 as usize, clauses.len());

        for _it in 0..self.bounded_resolve_iterations {
            for i in l..r {
                for j in 0..r {
                    let first = clauses[i].clone();
                    let second = clauses[j].clone();
                    Self::try_resolve(&mut clauses, g, first, second, s);
                }
            }

            if clauses.len() > max_clauses {
                while clauses.len() > max_clauses {
                    let c = clauses.pop().unwrap();
                    g.get_clauses().remove(&c);
                    
                }
            }

            (l, r) = (r, clauses.len());
        }
    }

    fn try_resolve(clauses: &mut Vec<Vec<Literal>>, g: &mut CNF, a: Vec<Literal>, b: Vec<Literal>, s: usize) {
        for l in b.iter().copied() {
            let resolvent = Self::resolve(a.clone(), b.clone(), l);

            if resolvent.is_none() {
                continue;
            }

            let resolvent = resolvent.unwrap();
            if resolvent.len() > s {
                continue;
            }

            if g.clauses().contains(&resolvent) {
                continue;
            }

            g.get_clauses().insert(resolvent.clone());
            clauses.push(resolvent);
        }
    }


    fn resolve(mut a: Vec<Literal>, mut b: Vec<Literal>, l: Literal) -> Option<Vec<Literal>> {
        if a.contains(&l.neg()) && b.contains(&l) {
            swap(&mut a, &mut b);
        } else if !a.contains(&l) || !b.contains(&l.neg()) {
            return None;
        }

        a = Self::remove_literal(a, l);
        b = Self::remove_literal(b, l.neg());

        let mut result = [a, b].concat();
        result.sort();
        result.dedup();
        Some(result)
    }

    fn remove_literal(clause: Vec<Literal>, literal: Literal) -> Vec<Literal> {
        clause.iter().cloned().filter(|l| l != &literal).collect()
    }

    fn search(g: &mut CNF, iterations: usize) -> Option<Vec<Literal>> {
        for _it in 1..=iterations {
            let mut pi: Vec<usize> = (1..=g.var_count()).collect();
            pi.shuffle(&mut rand::thread_rng());

            let y: Vec<bool> = (1..=g.var_count())
                .map(|_| rand::thread_rng().sample(&Uniform::new(0, 2)) == 1)
                .collect();

            let u = Self::modify(g.clone(), &pi, &y);
            
            if g.eval(u.clone()) {
                return Some(u);
            }
        }

        None
    }

    fn modify(mut g: CNF, pi: &Vec<usize>, y: &Vec<bool>) -> Vec<Literal> {
        let n = pi.len();
        let mut eval_set = Vec::<Literal>::new();

        for i in 0..n {
            let var = pi[i] as i32;

            let literal;
            match Self::check_for_unit_clause(&mut g, Literal::from_int(var)) {
                Some(true) => {
                    literal = Literal::from_int(var);
                },
                Some(false) => {
                    literal = Literal::from_int(-var);
                },
                None => {
                    literal = Literal{ var: var as usize, sign: !y[i] };
                }
            }

            Self::propagate(&mut g, literal);
            eval_set.push(literal);
        }

        eval_set
    }

    fn check_for_unit_clause(g: &mut CNF, l: Literal) -> Option<bool> {
        for c in g.get_clauses().iter() {
            if c.len() != 1 {
                continue;
            }

            if c.contains(&l) {
                return Some(true)
            }

            if c.contains(&l.neg()) {
                return Some(false);
            }
        }
        None
    }

    fn propagate(g: &mut CNF, l: Literal) {
        let mut to_remove: Vec::<Vec::<Literal>> = vec![];
        let mut to_add: Vec::<Vec::<Literal>> = vec![];

        for c in g.get_clauses().iter() {
            if c.contains(&l) {
                to_remove.push(c.clone());
                continue
            }

            if c.contains(&l.neg()) {
                to_remove.push(c.clone());
                to_add.push(c.iter().cloned()
                    .filter(|v| v != &l.neg())
                    .collect()
                );

                continue
            }
        }

        for c in to_remove {
            g.get_clauses().remove(&c);
        }

        for c in to_add {
            g.get_clauses().insert(c);
        }
    }
}