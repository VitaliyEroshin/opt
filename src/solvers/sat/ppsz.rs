use std::collections::{HashSet, HashMap};
use std::{mem::swap};

use rand::seq::SliceRandom;
use rand::{distributions::Uniform, Rng};

pub use super::solver::{Solver, Error};
use crate::p::cnf::{CNF, Literal};

#[derive(Clone)]
pub struct PPSZ {
    max_clauses: usize,
    max_resolve_iterations: usize,
    max_search_iterations: usize,
    max_clause_size: usize,
    bounded_resolve_iterations: usize,

    potential_resolve_clauses_with_literal: HashMap<Literal, HashSet<usize>>
}

#[derive(Clone)]
struct ExtendedCNF {
    clauses_set: HashSet<Vec<Literal>>,
    clauses: Vec<Vec<Literal>>,
    clauses_with_literal: HashMap<Literal, HashSet<usize>>,
    total_clauses: usize,
    unsatisified_clauses: usize,
    unit_clauses: HashSet<Literal>,
}

impl ExtendedCNF {
    pub fn from_cnf(cnf: &mut CNF) -> ExtendedCNF {
        let mut clauses = Vec::<Vec<Literal>>::new();
        let mut clauses_with_literal = HashMap::<Literal, HashSet<usize>>::new();
        let mut unit_clauses = HashSet::<Literal>::new();

        for c in cnf.get_clauses().iter() {
            if c.len() == 1 {
                unit_clauses.insert(c[0]);
                continue;
            }

            for l in c.iter() {
                clauses_with_literal.entry(*l)
                    .or_insert(HashSet::new())
                    .insert(clauses.len());
            }

            clauses.push(c.clone());
        }

        let total_clauses = clauses.len() + unit_clauses.len();

        ExtendedCNF {
            clauses_set: cnf.get_clauses().clone(),
            clauses: clauses,
            clauses_with_literal: clauses_with_literal,
            total_clauses: total_clauses,
            unsatisified_clauses: total_clauses,
            unit_clauses: unit_clauses,
        }
    }

    pub fn apply(&mut self, l: Literal) -> Option<Error> {
        if self.unit_clauses.contains(&l.neg()) {
            return Some(Error::new("Unsatisfiable"));
        }

        self.unsatisified_clauses -= self.unit_clauses.remove(&l) as usize;

        let mut to_remove = Vec::<(Literal, usize)>::new();
        let mut to_add = Vec::<(Literal, usize)>::new();

        let clauses_containing_l = self.clauses_with_literal
            .entry(l.clone())
            .or_insert(HashSet::new())
            .iter();

        for c in clauses_containing_l {
            let clause = &self.clauses[*c];

            for literal in clause.iter() {
                to_remove.push((literal.clone(), *c));
            }

            self.unsatisified_clauses -= 1;
        }

        let clauses_containing_not_l = self.clauses_with_literal
            .entry(l.neg())
            .or_insert(HashSet::new())
            .iter();

        for c in clauses_containing_not_l {
            let clause = self.clauses[*c].clone();

            for literal in clause.iter() {
                to_remove.push((literal.clone(), *c));
            }

            let resolved_clause = Self::clause_without_literal(clause, l);

            if self.clauses_set.contains(&resolved_clause) {
                continue;
            }

            self.unsatisified_clauses += 1;

            if resolved_clause.len() == 1 {
                self.unit_clauses.insert(resolved_clause[0]);
                continue;
            }

            for l in resolved_clause.iter() {
                to_add.push((*l, self.clauses.len()));
            }

            self.clauses.push(resolved_clause.clone());
            self.clauses_set.insert(resolved_clause);
        }

        for (l, index) in to_remove.into_iter() {
            self.get_clauses_with(l).remove(&index);
        }

        for (l, index) in to_add.into_iter() {
            self.get_clauses_with(l).insert(index);
        }

        None
    }

    pub fn get_literals(&mut self) -> Vec<Literal> {
        let mut literals = HashSet::<Literal>::new();

        for l in self.clauses_with_literal.keys().into_iter() {
            let literal = l.clone();

            literals.insert(literal);
            literals.insert(literal.neg());
        }
    
        literals.into_iter().collect()
    }

    pub fn clause_without_literal(clause: Vec<Literal>, l: Literal) -> Vec<Literal> {
        clause
            .into_iter()
            .filter(|literal| *literal != l && *literal != l.neg())
            .collect()
    }

    pub fn get_clauses_with(&mut self, l: Literal) -> &mut HashSet<usize> {
        self.clauses_with_literal
            .entry(l)
            .or_insert(HashSet::new())
    }

    pub fn clauses_with(&self, l: Literal) -> Option<&HashSet<usize>> {
        self.clauses_with_literal
            .get(&l)
    }

    pub fn contains(&self, clause: &Vec<Literal>) -> bool {
        if clause.len() == 1 {
            return self.unit_clauses.contains(&clause[0]);
        }
        self.clauses_set.contains(clause)
    }

    pub fn add_clause(&mut self, clause: Vec<Literal>) -> bool {
        if self.contains(&clause) {
            return false;
        }

        self.unsatisified_clauses += 1;
        self.total_clauses += 1;

        if clause.len() == 1 {
            self.unit_clauses.insert(clause[0]);
            return true;
        }

        for l in clause.iter() {
            let index = self.clauses.len();
            self.get_clauses_with(l.clone()).insert(index);
        }

        self.clauses.push(clause.clone());
        self.clauses_set.insert(clause);

        return true;
    }
}

impl Solver for PPSZ {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error> {
        let mut s = (*self).clone();

        let n = cnf.var_count();
        s.max_clauses = n * n * n / 5;  // Magic constant here

        Self::solve_ppsz(&mut s, cnf)
    }
}

impl PPSZ {
    pub fn new() -> PPSZ {
        PPSZ {
            max_clauses: 15000,
            max_resolve_iterations: 40,
            max_search_iterations: 1000,
            max_clause_size: 3,
            bounded_resolve_iterations: 2,

            potential_resolve_clauses_with_literal: HashMap::<Literal, HashSet<usize>>::new()
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

    pub fn solve_ppsz(&mut self, mut cnf: CNF) -> Result<Vec<Literal>, Error> {
        let mut ext_cnf = ExtendedCNF::from_cnf(&mut cnf);

        self.potential_resolve_clauses_with_literal = ext_cnf.clauses_with_literal.clone();

        for _i in 0..self.max_resolve_iterations {
            self.bounded_resolve(&mut ext_cnf, self.max_clause_size);

            if ext_cnf.unsatisified_clauses > self.max_clauses {
                break;
            }
        }

        match Self::search(&mut cnf, &mut ext_cnf, self.max_search_iterations) {
            None => {},
            Some(eval_set) => {
                return Ok(eval_set);
            }
        }
        return Err(Error::new("Unsatisfiable"));
    }

    fn bounded_resolve(&mut self, g: &mut ExtendedCNF, s: usize) {
        let literals = g.get_literals();

        let mut new_clauses = Vec::<Vec<Literal>>::new();

        for l in literals.into_iter() {
            
            let with_l;
            match self.potential_resolve_clauses_with_literal.get(&l) {
                None => { continue },
                Some(clauses) => { with_l = clauses }
            };

            let with_not_l;
            match g.clauses_with(l.neg()) {
                None => { continue },
                Some(clauses) => { with_not_l = clauses}
            }

            for i in with_l.iter() {
                for j in with_not_l.iter() {
                    let a = &g.clauses[*i];
                    let b = &g.clauses[*j];

                    let resolvent = Self::resolve(a.clone(), b.clone(), l);

                    if resolvent.is_none() {
                        continue;
                    }

                    let resolvent = resolvent.unwrap();
                    if resolvent.len() > s {
                        continue;
                    }

                    if Self::is_taut(&resolvent) {
                        continue;
                    }

                    new_clauses.push(resolvent);
                }
            }
        }

        self.potential_resolve_clauses_with_literal.clear();
        for clause in new_clauses.iter() {
            if g.contains(clause) {
                continue;
            }

            if clause.len() > 1 {
                for l in clause.iter() {
                    self.potential_resolve_clauses_with_literal
                        .entry(l.clone())
                        .or_insert(HashSet::new())
                        .insert(g.clauses.len());
                }
            }
            
            g.add_clause(clause.clone());
        }

        ()
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

    fn is_taut(a: &Vec<Literal>) -> bool {
        for i in a.iter() {
            for j in a.iter() {
                if i == &j.neg() {
                    return true;
                }
            }
        }

        return false;
    }

    fn remove_literal(clause: Vec<Literal>, literal: Literal) -> Vec<Literal> {
        clause.iter().cloned().filter(|l| l != &literal).collect()
    }

    fn search(g: &mut CNF, ext_g: &mut ExtendedCNF, iterations: usize) -> Option<Vec<Literal>> {
        for _it in 1..=iterations {
            let mut pi: Vec<usize> = (1..=g.var_count()).collect();
            pi.shuffle(&mut rand::thread_rng());

            let y: Vec<bool> = (1..=g.var_count())
                .map(|_| rand::thread_rng().sample(&Uniform::new(0, 2)) == 1)
                .collect();

            let u = Self::modify(ext_g.clone(), &pi, &y);
            
            if g.eval(u.clone()) {
                return Some(u);
            }
        }

        None
    }

    fn modify(mut g: ExtendedCNF, pi: &Vec<usize>, y: &Vec<bool>) -> Vec<Literal> {
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

            g.apply(literal);
            eval_set.push(literal);
        }

        eval_set
    }

    fn check_for_unit_clause(g: &mut ExtendedCNF, l: Literal) -> Option<bool> {
        if g.unit_clauses.contains(&l) {
            return Some(true)
        }

        if g.unit_clauses.contains(&l.neg()) {
            return Some(false);
        }

        None
    }
}