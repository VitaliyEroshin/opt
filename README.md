# Opt
A little rust framework

# How to run?
This is a framework, you should not run it, lol!

However, you can run some tests from `utils` folder.
### SAT tests
The SAT solving pipeline is located in `./src/bin/test_sat_solver.rs`, you can modify it if you want. Testing can be produced by running a python script, that reads and parses `.cnf` files in the `./utils/testcases` folder. It checks whether the algorithm found a proper evaluation set or not and prints the elapsed time.

```shell
# Assuming we are in the repository folder

cd utils
python3 test.py
```

Also, please, make sure, `cargo` is installed on your device.

# Primitives
### `opt::p::sat::Literal`
A trivially-copyable primitive that represents (*ha-ha*) a literal! Contains index of variable and whether it is negated or not. Literal can be represented as signed integer, where absolute value is an index of the variable, and sign is negation.

```rust
  // Makes literal from integer. Absolute value is a variable index, sign is a sign.
pub fn from_int(i: i32) -> Literal

  // Makes literal from string, same as from_int but parses the string
pub fn new(s: &str) -> Result<Literal, std::num::ParseIntError>

  // Makes negated literal
pub fn neg(&self) -> Literal

  // Returns index of corresponding variable
pub fn get_var(&self) -> usize

  // Returns the sign
pub fn is_negative(&self) -> bool
```

### `opt::p::sat::CNF`
Implements CNF - [Conjunctive Normal Form](https://en.wikipedia.org/wiki/Conjunctive_normal_form). Contains a set of clauses, each of which consists of literals.

```rust
  // Makes empty CNF
pub fn new() -> CNF

  // Returns variable count (variable with the largest index)
pub fn var_count(&self) -> usize

  // Adds the clause to CNF
pub fn add_clause(&mut self, mut clause: Vec<Literal>)

  // Returns mutable reference to the set of clauses
pub fn get_clauses(&mut self) -> &mut HashSet<Vec<Literal>>

  // Returns immutable reference to clauses
pub fn clauses(&self) -> &HashSet<Vec<Literal>>

  // Evaluates CNF on eval_vec. True if CNF is satisfied
pub fn eval(&self, eval_vec: Vec<Literal>) -> bool
```

# Solvers
For now there are only SAT solvers, some more can be added (*or not*) in the future
### `opt::solvers::sat`
Each SAT solver implements the following trait:
```rust
pub trait Solver {
    fn solve(&self, cnf: CNF) -> Result<Vec<Literal>, Error>;
}
```
Basically, it takes CNF and finds satisfiable evaluation set of literals. If something goes wrong, solve returns `Error` with the text of fail. **UNSAT = Fail**. So, there are some implemented algorithms for SAT solving:
### `opt::solvers::sat::ppsz`
Reference:
 - [**Original PPSZ algorithm (2005)**](https://cseweb.ucsd.edu/~paturi/myPapers/pubs/PaturiPudlakSaksZane_2005_jacm.pdf)

The first phase of algorithm is making a **bounded_resolution**. Here we produce some more clauses in our CNF using [resolutions](https://en.wikipedia.org/wiki/Resolution_(logic)). The bound is a size of clause.

The second phase is random search. We try to assign a random value to each variable in random order. After each assignment we can simplify the CNF. Sometimes, a value of variable is defined (CNF has a *unit-clause* with a single literal), then we assign the defined value.

If we found the evaluation set that makes clause set of CNF empty, then we found the correct evaluation set.
### `opt::solvers::sat::dpll`
Reference:
 - [**Wiki - DPLL algorithm**](https://en.wikipedia.org/wiki/DPLL_algorithm)

A backtracking algorithm, which, unfortunately, is broken now. I'm sorry. Going to fix it asap
# IO
### `opt::io::cnf`
There are several ways to get `CNF`s. You can do it, obviously, through io using following methods:
```rust
  // Reads CNF from the file
pub fn get_cnf_from_file(path: &str) -> Result<CNF, std::io::Error>

  // Reads CNF from stdin
pub fn get_cnf_from_stdin() -> Result<CNF, std::io::Error>
```
The format is pretty simple: 
- Very first line contains the number of clauses `N`
- Next `N` lines contain the clauses - signed integers separated by spaces

Example of proper CNF:
```
2
1 2 3
1 -2 3
```

The other way to gen `CNF` is generation.
```rust
  // Returns CNF with given properties. Very stupid algo, CNF can be UnSAT!
pub fn get_benchmark_cnf(variables: usize, clauses: usize, var_in_clauses: usize) -> CNF
```
