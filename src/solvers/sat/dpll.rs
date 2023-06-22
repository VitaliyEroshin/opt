use std::collections::{HashSet};

pub use super::solver::{Solver, Error};
use crate::p::cnf::{CNF, Literal};

pub struct DPLL {}

impl Solver for DPLL {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error> {
        Self::solve_dpll(cnf.clone())
    }
}

impl DPLL {
    fn simplify_cnf(cnf: &mut CNF) -> Result<Vec<Literal>, Error> {
        let mut eval_set = Vec::<Literal>::new();

        match Self::unit_propagation(cnf) {
            Err(e) => {
                return Err(e)
            },
            Ok(mut eval) => {
                eval_set.append(&mut eval);
            }
        }

        Self::cnf_normalization(cnf);

        let mut eval_subset = Self::pure_literal_ellimination(cnf);
        eval_set.append(&mut eval_subset);

        return Ok(eval_set);
    }

    fn solve_dpll(mut cnf: CNF) -> Result<Vec<Literal>, Error> {
        if Self::has_empty_clause(cnf.get_clauses()) {
            return Err(Error::new("Unsatisfied"));
        }

        let mut eval_set = Vec::<Literal>::new();

        let mut simplified = true;
        while simplified {
            match Self::simplify_cnf(&mut cnf) {
                Err(e) => {
                    return Err(e)
                },
                Ok(mut eval_subset) => {
                    if eval_subset.is_empty() {
                        simplified = false;
                    }

                    eval_set.append(&mut eval_subset);
                }
            }

            if cnf.get_clauses().is_empty() {
                return Ok(eval_set);
            }

            if Self::has_empty_clause(cnf.get_clauses()) {
                return Err(Error::new("Unsatisfied"));
            }
        }
        
        let l;

        match Self::get_any_literal(&mut cnf) {
            Some(x) => { l = x },
            None => { return Err(Error::new("Unsatisfied")) }
        }
    
        let (positive, negative) = Self::eval_on_literal(&mut cnf, l);

        Self::add_clauses(&mut cnf, &positive);

        match Self::solve_dpll(cnf.clone()) {
            Ok(mut eval_subset) => {
                eval_set.append(&mut eval_subset);
                eval_set.push(l.neg());

                return Ok(eval_set);
            },
            Err(_) => {}
        }

        Self::remove_clauses(&mut cnf, &positive);
        Self::add_clauses(&mut cnf, &negative);

        match Self::solve_dpll(cnf.clone()) {
            Ok(mut eval_subset) => {
                eval_set.append(&mut eval_subset);
                eval_set.push(l);

                return Ok(eval_set);
            },
            Err(_) => {}
        }

        Err(Error::new("Unsatisfied"))
    }

    fn is_unit_clause(clause: &Vec<Literal>) -> bool {
        return clause.len() == 1;
    }

    fn unit_propagation(cnf: &mut CNF) -> Result<Vec<Literal>, Error> {
        let mut unit_clauses = HashSet::<Literal>::new();
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();

        for clause in clauses.iter() {
            if !Self::is_unit_clause(&clause) {
                continue;
            }

            if unit_clauses.contains(&clause[0].neg()) {
                return Err(Error::new("Unsatisfiable"));
            }

            unit_clauses.insert(clause[0]);
        }

        let mut to_remove = Vec::<Vec<Literal>>::new();
        let mut to_add = Vec::<Vec<Literal>>::new();

        for clause in clauses.iter() {
            if Self::contains_literal_from_set(clause, &unit_clauses) {
                to_remove.push(clause.clone());
                continue;
            }

            if !Self::can_resolve_with_set(clause, &unit_clauses) {
                continue;
            }

            to_remove.push(clause.clone());
            let new_clause = clause
                .iter()
                .cloned()
                .filter(|literal| !unit_clauses.contains(literal) && !unit_clauses.contains(&literal.neg()))
                .collect();

            to_add.push(new_clause);
        }

        for clause in to_remove.iter() {
            clauses.remove(clause);
        }

        for clause in to_add.into_iter() {
            clauses.insert(clause);
        }
        
        return Ok(unit_clauses.into_iter().collect());
    }

    fn can_resolve_with_set(clause: &Vec<Literal>, set: &HashSet<Literal>) -> bool {
        for l in clause.iter() {
            if set.contains(&l.neg()) {
                return true;
            }
        }
        false
    }

    fn contains_literal_from_set(clause: &Vec<Literal>, set: &HashSet<Literal>) -> bool {
        for l in clause.iter() {
            if set.contains(l) {
                return true;
            }
        }
        return false;
    }

    fn cnf_normalization(cnf: &mut CNF) {
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();
        let mut to_remove = Vec::<Vec<Literal>>::new();

        for clause in clauses.iter() {
            let mut known_literals = HashSet::<Literal>::new();

            for literal in clause.iter() {
                if known_literals.contains(&literal.neg()) {
                    to_remove.push(clause.clone());
                    break;
                }
                known_literals.insert(literal.clone());
            }
        }

        for c in to_remove.iter() {
            clauses.remove(c);
        }
    }

    fn pure_literal_ellimination(cnf: &mut CNF) -> Vec<Literal> {
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();

        let mut known_literals = HashSet::<Literal>::new();
        let mut not_a_pure_literals = HashSet::<Literal>::new();

        for clause in clauses.iter() {
            for literal in clause {
                if known_literals.contains(&literal.neg()) {
                    not_a_pure_literals.insert(literal.clone());
                    not_a_pure_literals.insert(literal.neg());
                } else {
                    known_literals.insert(literal.clone());
                }
            }
        }

        let mut pure_literals = HashSet::<Literal>::new();
        
        for l in known_literals.into_iter() {
            if !not_a_pure_literals.contains(&l) {
                pure_literals.insert(l);
            }
        }

        let mut to_remove = Vec::<Vec<Literal>>::new();
        for clause in clauses.iter() {
            for literal in clause {
                if pure_literals.contains(literal) {
                    to_remove.push(clause.clone());
                    break;
                }
            }
        }

        for clause in to_remove.iter() {
            clauses.remove(clause);
        }

        pure_literals.into_iter().collect()
    }

    fn remove_literal(clause: &Vec<Literal>, literal: &Literal) -> Vec<Literal> {
        clause
            .iter()
            .cloned()
            .filter(|l| l != literal)
            .collect()
    }

    fn eval_on_literal(cnf: &mut CNF, l: Literal) -> (Vec<Vec<Literal>>, Vec<Vec<Literal>>) {
        let mut have_positive = Vec::<Vec<Literal>>::new();
        let mut have_negative = Vec::<Vec<Literal>>::new();

        let mut to_remove = Vec::<Vec<Literal>>::new();

        for clause in cnf.clauses().iter() {
            if clause.contains(&l) {
                let new_clause = Self::remove_literal(clause, &l);
                if !cnf.clauses().contains(&new_clause) {
                    have_positive.push(new_clause);
                }
                
                to_remove.push(clause.clone());

            } else if clause.contains(&l.neg()) {
                let new_clause = Self::remove_literal(clause, &l.neg());
                if !cnf.clauses().contains(&new_clause) {
                    have_negative.push(new_clause);
                }
        
                to_remove.push(clause.clone());

            }
        }

        for clause in to_remove.iter() {
            cnf.get_clauses().remove(clause);
        }

        (have_positive, have_negative)
    }

    fn get_any_literal(c: &mut CNF) -> Option<Literal> {
        for clause in c.get_clauses().iter() {
            if !clause.is_empty() {
                return Some(clause[0]);
            }
        }
        return None;
    }

    fn remove_clauses(c: &mut CNF, clauses: &Vec<Vec<Literal>>) {
        for clause in clauses.iter() {
            c.get_clauses().remove(clause);
        }
    }

    fn add_clauses(c: &mut CNF, clauses: &Vec<Vec<Literal>>) {
        for clause in clauses.iter() {
            c.get_clauses().insert(clause.clone());
        }
    }

    fn has_empty_clause(clauses: &HashSet<Vec<Literal>>) -> bool {
        for c in clauses.iter() {
            if c.is_empty() {
                return true;
            }
        }

        return false;
    }
}