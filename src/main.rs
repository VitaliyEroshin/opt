mod formula;
use crate::formula::cnf::{CNF, SATSolver, Literal};

fn print_cnf(cnf: &mut CNF) {
    let clauses = cnf.get_clauses();
    for clause in clauses {
        println!("{:?}", clause);
    }
}

fn unit_propagation_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal { var: 0, sign: true }]);
    cnf.add_clause(vec![Literal { var: 1, sign: true }]);
    cnf.add_clause(vec![Literal { var: 2, sign: true }, Literal {var: 0, sign: false}]);

    
    cnf = SATSolver::unit_propagation(cnf);

    assert_eq!(cnf.get_clauses(), &vec![vec![Literal { var: 2, sign: true}]]);
    println!("Unit propagation test passed!");
}

fn normalization_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal { var: 0, sign: true }]);
    cnf.add_clause(vec![Literal { var: 1, sign: true }, Literal { var: 1, sign: false}]);
    cnf.add_clause(vec![Literal { var: 2, sign: true }, Literal {var: 0, sign: false}]);
    
    cnf = SATSolver::normalize_cnf(cnf);

    assert_eq!(cnf.get_clauses(), &vec![
        vec![Literal { var: 0, sign: true }],
        vec![Literal { var: 2, sign: true }, Literal { var: 0, sign: false }]
    ]);
}

fn pure_literal_ellimination_test() {
    let mut cnf = CNF::new();
    cnf.add_clause(vec![Literal {var: 0, sign: true}]);
    cnf.add_clause(vec![Literal {var: 0, sign: false}, Literal {var: 1, sign: true}]);

    cnf = SATSolver::pure_literal_ellimination(cnf);
    
    assert_eq!(cnf.get_clauses(), &vec![vec![Literal {var: 0, sign: true}]]);
}

fn main() {

}
