# Opt
A little rust framework

# How to run?
This is a framework, you should not run it, lol!

However, you can run some tests from `utils` folder.
### SAT tests
Testing can be produced by running a python script. You need to provide which solver do you want to test using `--solver` option. It can be `dpll` or `ppsz`. You can optionally provide a path to folder with you own `.cnf` tests using `--testcases` option.

For example:
```shell
# Assuming we are in the repository folder

cd utils
python3 test.py --solver dpll
```

You can also run solvers as cargo targets like so:
```shell
cargo run --bin test_ppsz_solver < input.txt
```

Also, please, make sure, `cargo` is installed on your device.

# Primitives
### `opt::p::cnf::Literal`
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

### `opt::p::cnf::CNF`
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
### [`opt::solvers::sat::ppsz`](https://github.com/VitaliyEroshin/opt/blob/main/src/solvers/sat/ppsz.rs)
**Reference:**
 - [**Original PPSZ algorithm (2005)**](https://cseweb.ucsd.edu/~paturi/myPapers/pubs/PaturiPudlakSaksZane_2005_jacm.pdf)

**Algorithm overview:**

The first phase of algorithm is making a **bounded_resolution**. Here we produce some more clauses in our CNF using [resolutions](https://en.wikipedia.org/wiki/Resolution_(logic)). The bound is a size of clause.

The second phase is random search. We try to assign a random value to each variable in random order. After each assignment we can simplify the CNF. Sometimes, a value of variable is defined (CNF has a *unit-clause* with a single literal), then we assign the defined value.

If we found the evaluation set that makes clause set of CNF empty, then we found the correct evaluation set.

**Tuning:**
There are some parameters that can be changed via setter methods of PPSZ:
```rust
pub struct PPSZ {
    max_clauses: usize,  // max clause count can be reached during bounded_resolution phase
    max_resolve_iterations: usize,  // max number of bounded_resolve + search iterations
    max_search_iterations: usize,  // max number of search iterations
    max_clause_size: usize,  // actual resolution bound
    bounded_resolve_iterations: usize,  // number of iterations made in bounded_resolve
}
```

### [`opt::solvers::sat::dpll`](https://github.com/VitaliyEroshin/opt/blob/main/src/solvers/sat/dpll.rs)
**Reference:**
 - [**Wiki - DPLL algorithm**](https://en.wikipedia.org/wiki/DPLL_algorithm)

**Algorithm overview:**

Firstly algorithm simplifies the CNF:
 - Unit clauses propagation: if there is a clause with single variable then value of that variable is defined. Algorithm assigns the value then removes satisfied clauses and resolves clauses with a logical opposite of the variable.
 - Pure literal ellimination: if there is a literal in CNF which negation is not presented then it's value defined. Algorithm can simplify clauses yet again.
 - If there are no clauses left then we found satisfiable evaluation set
 - If there is empty clause then there is no satisfiable evaluation set

Now, when CNF is simplified, we can branch and solve recursively. Algorithm takes random literal `l` represented in CNF and solves SAT for CNF for `l = true` and for `l = false` separately
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
