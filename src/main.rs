mod formula;
use formula::propositional::PropositionalFormula;

fn main() {
    let mut p = PropositionalFormula::new("(1 or not 3) and (4 or 6)".to_string());
    p.parse();
    println!("{:?}", p.get_cnf().unwrap().get_clauses());
}
