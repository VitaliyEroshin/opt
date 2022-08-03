mod formula;
use crate::formula::cnf::{CNF, SATSolver, Literal};
use std::collections::HashSet;

fn print_cnf(cnf: &mut CNF) {
    let clauses = cnf.get_clauses();
    for clause in clauses.iter() {
        println!("{:?}", clause);
    }
}

fn unit_propagation_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal { var: 0, sign: true }]);
    cnf.add_clause(vec![Literal { var: 1, sign: true }]);
    cnf.add_clause(vec![Literal { var: 2, sign: true }, Literal {var: 0, sign: false}]);

    
    cnf = SATSolver::unit_propagation(cnf);

    let result: Vec<_> = cnf.get_clauses().clone().into_iter().collect();
    assert_eq!(result, vec![vec![Literal { var: 2, sign: true}]]);
    println!("Unit propagation test passed!");
}

fn normalization_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal { var: 0, sign: true }]);
    cnf.add_clause(vec![Literal { var: 1, sign: true }, Literal { var: 1, sign: false}]);
    cnf.add_clause(vec![Literal { var: 2, sign: true }, Literal {var: 0, sign: false}]);
    
    cnf = SATSolver::normalize_cnf(cnf);

    let result: Vec<_> = cnf.get_clauses().clone().into_iter().collect();
    assert_eq!(result, vec![
        vec![Literal { var: 0, sign: true }],
        vec![Literal { var: 2, sign: true }, Literal { var: 0, sign: false }]
    ]);
}

fn pure_literal_ellimination_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal {var: 0, sign: true}]);
    cnf.add_clause(vec![Literal {var: 0, sign: false}, Literal {var: 1, sign: true}]);

    cnf = SATSolver::pure_literal_ellimination(cnf);

    let result: Vec<_> = cnf.get_clauses().clone().into_iter().collect();
    assert_eq!(result, vec![vec![Literal {var: 0, sign: true}]]);
}


use std::io::{self, BufRead};
use std::fs::File;

fn get_cnf_from_file() -> CNF {
    let file = File::open("./test.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut cnf = CNF::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut clause = Vec::new();
        for literal in line.split(" ") {
            let mut literal = literal.parse::<i32>().unwrap();
            let sign = literal < 0;
            if sign {
                literal = literal.abs();
            }

            clause.push(Literal {var: literal as usize, sign: sign});
        }
        clause.pop();
        cnf.add_clause(clause);
    }
    cnf
}

fn main() {
    unit_propagation_test();
    normalization_test();
    pure_literal_ellimination_test();

    let mut cnf = get_cnf_from_file();
    let (res, cnf) = SATSolver::solve(cnf);
    println!("{:?}", res);
}
