mod formula;
use crate::formula::cnf::{CNF, SATSolver, Literal};

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
    let cnf = get_cnf_from_file();
    let (res, _cnf) = SATSolver::solve(cnf);
    println!("{:?}", res);
}
