use opt::solvers::sat::{dpll};

mod support {
    pub mod solve_sat;
}

fn main() {
    let solver = dpll::DPLL::new();
    support::solve_sat::solve_sat(solver);
}
