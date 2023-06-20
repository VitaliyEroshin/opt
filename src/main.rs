mod formula;
// use formula::propositional::PropositionalFormula;

fn main() {
    let cnf = formula::cnf_tools::get_cnf_from_stdin();

    match cnf {
        Ok(cnf) => {
            println!("{:}", cnf);
        },
        Err(err) => {
            println!("Error occured while parsing CNF: {}", err.to_string());
            return;
        }
    }
}
