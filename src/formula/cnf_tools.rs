use super::cnf::{CNF, Literal};
use std::io::{self, BufRead};
use std::fs::File;
use rand::Rng;

pub fn get_benchmark_cnf(variables: usize, clauses: usize, var_in_clauses: usize) -> CNF {
    let mut cnf = CNF::new();
    for _ in 0..clauses {
        let mut clause = Vec::new();
        for _ in 0..var_in_clauses {
            let mut rng = rand::thread_rng();
            let var = rng.gen_range(0..variables);
            let sign = rng.gen();
            clause.push(Literal { var, sign });
        }
        cnf.add_clause(clause);
    }
    return cnf;
}

fn parse_clause_from_line(s: String) -> Result<Vec<Literal>, std::io::Error> {
    let mut clause = Vec::new();

    for lit in s.split(" ") {
        match Literal::new(lit) {
            Ok(l) => {
                clause.push(l);
            },
            Err(_) => {
                return Err(std::io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to parse literal, must be signed integer"
                ))
            }
        }
    }
    
    Ok(clause)
}

fn read_cnf_from_buff<Stream: std::io::Read>(reader: io::BufReader<Stream>) -> Result<CNF, std::io::Error> {
    let mut cnf = CNF::new();

    for line in reader.lines() {
        let s = line?;

        cnf.add_clause(parse_clause_from_line(s)?);
    }

    Ok(cnf)
}

pub fn get_cnf_from_file(path: &str) -> Result<CNF, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    read_cnf_from_buff(reader)
}

pub fn get_cnf_from_stdin() -> Result<CNF, std::io::Error> {
    let reader = io::BufReader::new(io::stdin());

    read_cnf_from_buff(reader)
}