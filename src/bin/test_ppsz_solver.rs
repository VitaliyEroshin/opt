use opt::solvers::sat::{ppsz};

mod support {
    pub mod solve_sat;
}

fn main() {
    let solver = ppsz::PPSZ::new();
    support::solve_sat::solve_sat(solver);
}