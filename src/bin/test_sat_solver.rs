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

    let solver = solvers::sat::dpll::DPLL{};

    match solver.solve(c.clone()) {
        Ok(eval_set) => {
            for v in eval_set.iter() {
                print!("{:?} ", v);
            }
            println!("");
            println!("SAT: {:}", c.eval(eval_set))
        },
        Err(e) => {
            println!("Error when solving: {:}", e.what());
        }
    }
}
