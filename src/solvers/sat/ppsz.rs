use std::collections::HashSet;
use std::{mem::swap};

use rand::seq::SliceRandom;
use rand::{distributions::Uniform, Rng};

pub use super::solver::{Solver, Error};
use crate::p::cnf::{CNF, Literal};

pub struct PPSZ {}

impl Solver for PPSZ {
    fn solve(&self, mut cnf: CNF) -> Result<Vec<Literal>, Error> {
        Self::bounded_resolve(&mut cnf, 3);
        match Self::search(&mut cnf, 10000) {
            None => {
                return Err(Error::new("Unsatisfiable"))
            },
            Some(eval_set) => {
                return Ok(eval_set);
            }
        }
    }
}

impl PPSZ {
    fn bounded_resolve(g: &mut CNF, s: usize) -> usize {
        let mut new_clauses: HashSet<Vec<Literal>> = HashSet::new();

        for a in g.clauses().iter() {
            for b in g.clauses().iter() {
                Self::try_resolve(&mut new_clauses, a, b, s);
            }
        }

        let mut added = 0;

        for c in new_clauses.into_iter() {
            if g.get_clauses().insert(c) {
                added += 1;
            }
        }
    
        added
    }

    fn try_resolve(new_clauses: &mut HashSet<Vec<Literal>>, a: &Vec<Literal>, b: &Vec<Literal>, s: usize) {
        if a.len() > s || b.len() > s {
            return;
        }

        for l in b.iter().copied() {
            let resolvent = Self::resolve(a.clone(), b.clone(), l);

            if resolvent.is_none() {
                continue;
            }

            let resolvent = resolvent.unwrap();
            if resolvent.len() > s {
                continue;
            }

            new_clauses.insert(resolvent);
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
        for _ in 1..=iterations {
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