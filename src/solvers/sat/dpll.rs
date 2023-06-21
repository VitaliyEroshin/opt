use std::collections::{HashSet};
use std::mem::swap;

pub use super::solver::{Solver, Error};
use crate::p::cnf::{CNF, Literal};

pub struct DPLL {}

impl Solver for DPLL {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error> {
        let (sat, mut cnf) = Self::solve_dpll(cnf);
        
        if !sat {
            return Err(Error::new("Unsatisfiable"));
        }

        let mut eval_set: Vec<Literal> = vec![];

        // TODO: Could avoid cloning here?
        for mut clause in cnf.get_clauses().clone().into_iter() {
            eval_set.append(&mut clause);
        }
        
        Ok(eval_set)
    }
}

impl DPLL {
    fn solve_dpll(mut cnf: CNF) -> (bool, CNF) {
        let mut eval_set = Vec::<Literal>::new();
        (cnf, eval_set) = Self::unit_propagation(cnf, eval_set);
        cnf = Self::normalize_cnf(cnf);
        (cnf, eval_set) = Self::pure_literal_ellimination(cnf, eval_set);

        if cnf.get_clauses().is_empty() {
            for literal in eval_set {
                cnf.add_clause(vec![literal]);
            }
            return (true, cnf);
        }

        if cnf.has_empty_clause() {
            return (false, cnf);
        }
        
        let l: Literal;

        match Self::get_any_literal(&mut cnf) {
            None => {
                return (false, cnf);
            },
            Some(literal) => {
                l = literal;
            }
        }

        let (true_value_clauses, false_value_clauses) = Self::evaluate_on_literal(&mut cnf, l);
        cnf.remove_clauses_with_literal(l);
        cnf.add_clauses(&false_value_clauses);
        let mut result;

        (result, cnf) = Self::solve_dpll(cnf);
        if result {
            cnf.add_clause(vec![l]);
            for literal in eval_set {
                cnf.add_clause(vec![literal]);
            }
            return (result, cnf);
        }

        cnf.remove_clauses(&false_value_clauses);
        cnf.add_clauses(&true_value_clauses);

        (result, cnf) = Self::solve_dpll(cnf);
        if result {
            cnf.add_clause(vec![l.neg()]);
            for literal in eval_set {
                cnf.add_clause(vec![literal]);
            }
            return (result, cnf);
        }

        cnf.remove_clauses(&true_value_clauses);
        cnf.remove_clauses(&false_value_clauses);

        for clause in true_value_clauses {
            let mut new_clause = clause.clone();
            new_clause.push(l);
            cnf.add_clause(new_clause)
        }

        for clause in false_value_clauses {
            let mut new_clause = clause.clone();
            new_clause.push(l.neg());
            cnf.add_clause(new_clause)
        }


        (false, cnf)
    }

    fn is_unit_clause(clause: &Vec<Literal>) -> bool {
        return clause.len() == 1;
    }

    pub fn unit_propagation(mut cnf: CNF, mut eval_set: Vec<Literal>) -> (CNF, Vec<Literal>) {
        let mut unit_clauses = HashSet::<Literal>::new();
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();

        for clause in clauses.iter() {
            if Self::is_unit_clause(&clause) && !unit_clauses.contains(&clause[0].neg()) {
                unit_clauses.insert(clause[0].clone());
            }
        }

        let mut removed_clauses = HashSet::<Vec<Literal>>::new();
        let mut add_clauses = HashSet::<Vec<Literal>>::new();
        for clause in clauses.iter() {
            if Self::is_unit_clause(&clause) && unit_clauses.contains(&clause[0]) {
                removed_clauses.insert(clause.clone());
                eval_set.push(clause[0].clone());
                continue;
            }

            for index in 0..clause.len() {
                if unit_clauses.contains(&clause[index].neg()) {
                    let mut new_clause = clause.clone();
                    new_clause.swap_remove(index);

                    removed_clauses.insert(clause.clone());
                    add_clauses.insert(new_clause);
                    break;
                }
            }
        }

        for clause in removed_clauses.iter() {
            clauses.remove(clause);
        }

        for mut clause in add_clauses.into_iter() {
            clause.sort();
            clauses.insert(clause);
        }
        
        return (cnf, eval_set);
    }

    pub fn normalize_cnf(mut cnf: CNF) -> CNF {
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();
        
        let mut removed_clauses = HashSet::<Vec<Literal>>::new();
        for clause in clauses.iter() {
            let mut known_literals = HashSet::<Literal>::new();

            for literal in clause.iter() {
                if known_literals.contains(&literal.neg()) {
                    removed_clauses.insert(clause.clone());
                    break;
                }
                known_literals.insert(literal.clone());
            }
        }

        cnf.remove_clauses(&removed_clauses);
        cnf
    }

    pub fn pure_literal_ellimination(mut cnf: CNF, mut eval_set: Vec<Literal>) -> (CNF, Vec<Literal>) {
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();
        let mut known_literals = HashSet::<Literal>::new();
        let mut ellimination_whitelist = HashSet::<Literal>::new();

        for clause in clauses.iter() {
            for literal in clause {
                if known_literals.contains(&literal.neg()) {
                    ellimination_whitelist.insert(literal.clone());
                    ellimination_whitelist.insert(literal.neg());
                } else {
                    known_literals.insert(literal.clone());
                }
            }
        }

        let mut removed_clauses = HashSet::<Vec<Literal>>::new();
        for clause in clauses.iter() {
            for literal in clause {
                if !ellimination_whitelist.contains(&literal) {
                    removed_clauses.insert(clause.clone());
                    eval_set.push(literal.clone());
                    break;
                }
            }
        }

        cnf.remove_clauses(&removed_clauses);

        (cnf, eval_set)
    }

    fn remove_literal(mut clause: Vec<Literal>, literal: &Literal) -> Vec<Literal> {
        let mut index: usize = 0;
        while index != clause.len() {
            if clause[index] == *literal {
                clause.remove(index);
                return clause;
            }
            index += 1;
        }
        return clause;
    }

    pub fn resolute(mut a: Vec<Literal>, mut b: Vec<Literal>, l: Literal) -> Option<Vec<Literal>> {
        if a.contains(&l.neg()) && b.contains(&l) {
            swap(&mut a, &mut b);
        } else if !a.contains(&l) || !b.contains(&l.neg()) {
            return None;
        }
        a = Self::remove_literal(a, &l);
        b = Self::remove_literal(b, &l.neg());
        let mut result = [a, b].concat();
        result.sort();
        result.dedup();
        Some(result)
    }

    pub fn davis_putnam_procedure(mut cnf: CNF) -> CNF {
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();
        let mut all_literals = HashSet::<Literal>::new();

        for clause in clauses.iter() {
            for literal in clause {
                all_literals.insert(literal.clone());
            }
        }

        for l in all_literals {
            let mut new_clauses: HashSet<Vec<Literal>> = HashSet::new();
            let mut removed_clauses: HashSet<Vec<Literal>> = HashSet::new();

            for a in clauses.iter() {
                for b in clauses.iter() {
                    let resolvent = Self::resolute(a.clone(), b.clone(), l);
                    
                    if resolvent.is_some() {
                        let resolvent = &resolvent.unwrap();
                        new_clauses.insert(resolvent.to_vec());
                    }
                }
            }

            for clause in clauses.iter() {
                if clause.contains(&l) || clause.contains(&l.neg()) {
                    removed_clauses.insert(clause.clone());
                }
            }

            for clause in removed_clauses {
                clauses.remove(&clause);
            }

            let mut inserted = false;
            for mut clause in new_clauses {
                clause.sort();
                clauses.insert(clause.clone());
                inserted = true;
            }

            if inserted {
                break;
            }
        }

        cnf
    }

    pub fn evaluate_on_literal(cnf: &mut CNF, l: Literal) -> (HashSet<Vec<Literal>>, HashSet<Vec<Literal>>) {
        let mut true_value_clauses = HashSet::<Vec<Literal>>::new();
        let mut false_value_clauses = HashSet::<Vec<Literal>>::new();

        for clause in cnf.get_clauses().iter() {
            if clause.contains(&l) {
                let mut new_clause = clause.clone();
                new_clause = Self::remove_literal(new_clause, &l);
                true_value_clauses.insert(new_clause);
            } else if clause.contains(&l.neg()) {
                let mut new_clause = clause.clone();
                new_clause = Self::remove_literal(new_clause, &l.neg());
                false_value_clauses.insert(new_clause);
            }
        }

        return (true_value_clauses, false_value_clauses);
    }

    fn get_any_literal(c: &mut CNF) -> Option<Literal> {
        for clause in c.get_clauses().iter() {
            if !clause.is_empty() {
                return Some(clause[0]);
            }
        }
        return None;
    }
}