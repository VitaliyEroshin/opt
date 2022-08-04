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

pub struct SATSolver {}

impl SATSolver {
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

        for clause in removed_clauses.iter() {
            clauses.remove(clause);
        }

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

        for clause in removed_clauses.iter() {
            clauses.remove(clause);
        }

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
            let mut eval_set = Vec::<Literal>::new();
            println!("New iteration, cnf size is {}", cnf.get_clauses().len());
            (cnf, eval_set) = Self::unit_propagation(cnf, eval_set);
            cnf = Self::normalize_cnf(cnf);
            (cnf, eval_set) = Self::pure_literal_ellimination(cnf, eval_set);
            if cnf.get_clauses().is_empty() {
                return (true, cnf);
            }

            if cnf.has_empty_clause() {
                return (false, cnf);
            }
            cnf = Self::davis_putnam_procedure(cnf);
        }
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

    pub fn solve_dpll(mut cnf: CNF) -> (bool, CNF) {
        println!("New iteration, cnf size is {}", cnf.get_clauses().len());
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
        
        let l = cnf.get_literal().unwrap();

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

    pub fn solve(mut cnf: CNF) -> Option<CNF> {
        let SAT;
        (SAT, cnf) = Self::solve_dpll(cnf);
        if SAT {
            Some(cnf)
        } else {
            None
        }
    }

}