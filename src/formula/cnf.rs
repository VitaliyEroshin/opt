use std::{collections::HashSet, mem::swap, hash::Hash};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct Literal {
    pub var: usize,
    pub sign: bool,
}

impl Literal {
    pub fn neg(&self) -> Literal {
        return Literal {
            var: self.var,
            sign: !self.sign,
        };
    }
}

pub struct CNF {
    clauses: HashSet<Vec<Literal>>,
}

impl CNF {
    pub fn new() -> CNF {
        CNF {
            clauses: HashSet::new(),
        }
    }

    pub fn add_clause(&mut self, mut clause: Vec<Literal>) {
        clause.sort();
        self.clauses.insert(clause);
    }

    pub fn get_clauses(&mut self) -> &mut HashSet<Vec<Literal>> {
        return &mut self.clauses;
    }

    pub fn get_literal(&self) -> Option<Literal> {
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
}

pub struct SATSolver {}

impl SATSolver {
    fn is_unit_clause(clause: &Vec<Literal>) -> bool {
        return clause.len() == 1;
    }

    pub fn unit_propagation(mut cnf: CNF) -> CNF {
        let mut unit_clauses = HashSet::<Literal>::new();
        let clauses: &mut HashSet<Vec<Literal>> = cnf.get_clauses();

        for clause in clauses.iter() {
            if Self::is_unit_clause(&clause) {
                unit_clauses.insert(clause[0].clone());
            }
        }

        let mut removed_clauses = HashSet::<Vec<Literal>>::new();
        let mut add_clauses = HashSet::<Vec<Literal>>::new();
        for clause in clauses.iter() {
            if Self::is_unit_clause(&clause) {
                removed_clauses.insert(clause.clone());
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
        
        return cnf;
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

        for clause in removed_clauses.iter() {
            clauses.remove(clause);
        }

        cnf
    }

    pub fn pure_literal_ellimination(mut cnf: CNF) -> CNF {
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
                    break;
                }
            }
        }

        for clause in removed_clauses.iter() {
            clauses.remove(clause);
        }

        cnf
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

            for clause in removed_clauses.iter() {
                clauses.remove(clause);
            }

            let mut inserted = false;
            for mut clause in new_clauses.into_iter() {
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

    #[allow(dead_code)]
    pub fn solve_davis_putnam(mut cnf: CNF) -> (bool, CNF) {
        loop {
            println!("New iteration, cnf size is {}", cnf.get_clauses().len());
            cnf = Self::unit_propagation(cnf);
            cnf = Self::normalize_cnf(cnf);
            cnf = Self::pure_literal_ellimination(cnf);
            if cnf.get_clauses().is_empty() {
                return (true, cnf);
            }

            if cnf.has_empty_clause() {
                return (false, cnf);
            }
            cnf = Self::davis_putnam_procedure(cnf);
        }
    }

    pub fn evaluate_on_literal(cnf: &mut CNF, l: Literal) -> CNF {
        let mut new_cnf = CNF::new();
        for clause in cnf.get_clauses().iter() {
            if clause.contains(&l) {
                continue;
            } else if clause.contains(&l.neg()) {
                let mut new_clause = clause.clone();
                new_clause = Self::remove_literal(new_clause, &l.neg());
                new_cnf.add_clause(new_clause);
            } else {
                new_cnf.add_clause(clause.clone());
            }
        }

        new_cnf
    }

    pub fn solve_dpll(mut cnf: CNF) -> (bool, CNF) {
        println!("New iteration, cnf size is {}", cnf.get_clauses().len());
        cnf = Self::unit_propagation(cnf);
        cnf = Self::normalize_cnf(cnf);
        cnf = Self::pure_literal_ellimination(cnf);

        if cnf.get_clauses().is_empty() {
            return (true, cnf);
        }

        if cnf.has_empty_clause() {
            return (false, cnf);
        }
        
        let l;

        match cnf.get_literal() {
            Some(literal) => l = literal,
            None => return (true, cnf),
        }

        let on_true = Self::evaluate_on_literal(&mut cnf, l);
        let on_false = Self::evaluate_on_literal(&mut cnf, l.neg());

        let (on_true_result, _on_true_cnf) = Self::solve_dpll(on_true);
        if on_true_result {
            return (true, cnf);
        }

        let (on_false_result, _on_false_cnf) = Self::solve_dpll(on_false);
        (on_false_result, cnf)
    }

    pub fn solve(cnf: CNF) -> (bool, CNF) {
        Self::solve_dpll(cnf)
    }

}