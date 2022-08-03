use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Literal {
    pub var: usize,
    pub sign: bool,
}

pub struct CNF {
    clauses: Vec<Vec<Literal>>,
}

impl CNF {
    pub fn new() -> CNF {
        CNF {
            clauses: Vec::new(),
        }
    }

    pub fn add_clause(&mut self, clause: Vec<Literal>) {
        self.clauses.push(clause);
    }

    pub fn get_clauses(&mut self) -> &mut Vec<Vec<Literal>> {
        return &mut self.clauses;
    }
}

pub struct SATSolver {}

impl SATSolver {
    fn is_unit_clause(clause: &Vec<Literal>) -> bool {
        return clause.len() == 1;
    }

    fn get_negative_literal(literal: &Literal) -> Literal {
        return Literal {
            var: literal.var,
            sign: !literal.sign,
        };
    }

    pub fn unit_propagation(mut cnf: CNF) -> CNF {
        let mut unit_clauses = HashSet::<Literal>::new();
        let clauses: &mut Vec<Vec<Literal>> = cnf.get_clauses();

        for clause in clauses.iter() {
            if Self::is_unit_clause(&clause) {
                unit_clauses.insert(clause[0].clone());
            }
        }

        let mut index: usize = 0;
        while index != clauses.len() {
            let clause: &mut Vec<Literal> = &mut clauses[index];
            if Self::is_unit_clause(&clause) {
                clauses.swap_remove(index);
                continue;
            }
            
            let mut literal_index: usize = 0;
            while literal_index != clause.len() {
                let literal = &clause[literal_index];
                if unit_clauses.contains(&Self::get_negative_literal(&literal)) {
                    clause.swap_remove(literal_index);
                    continue;
                }
                literal_index += 1;
            }
            index += 1;
        }
        
        return cnf;
    }

    fn solve(mut cnf: CNF) -> (bool, CNF) {
        cnf = Self::unit_propagation(cnf);

        (true, cnf)
    }

}