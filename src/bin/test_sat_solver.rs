use opt::{io, p, solvers};

use solvers::sat::solver::*;

fn main() {
    let cnf = io::cnf::get_cnf_from_stdin();

    let c: p::cnf::CNF;
    match cnf {
        Ok(cnf) => {
            c = cnf;
        },
        Err(err) => {
            println!("Error occured while parsing CNF: {}", err.to_string());
            return;
        }
    }

    let solver = solvers::sat::ppsz::PPSZ{};

    match solver.solve(c.clone()) {
        Ok(eval_set) => {
            for v in eval_set.iter() {
                print!("{:?} ", v);
            }
        },
        Err(e) => {
            println!("Error when solving: {:}", e.what());
        }
    }
}
