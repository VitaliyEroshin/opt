# Logic elements implemented with Rust lang
## Propositional formulas
``` 
use formula::propositional::PropositionalFormula
```
So, basically, you can create propositional formula using round parentheses (for priority), binary/unary operators (**and**, **or**, **implies**, **xor**, **not**) and variables (integers). 
These are correct formulas: ```(1 or 2) and (3 or (4 and 6))```, ```not 6 or not(5) or 3```. Incorrect parentheses balance causes undefined behavior. **No guarantees here.**

```
 /* load the string representation of formula */
 let mut p = PropositionalFormula::new("(1 or not 3) and (4 or 6)".to_string());
 
 /* build a computation tree for the formula */ 
 p.parse();
```
## Conjunctive normal form (CNF)
```
use formula::cnf::{CNF, Literal}
use formula::cnf_tools
```
This section contains two primitives (**CNF** and **Literal**). CNF is just a set of clauses of literals. You can build them manually, bu applying ```cnf.add_clause(Vec<Literal>)``` method. Or you can get a CNF object from your PropositionalFormula using ```p.get_cnf()```, which returns ```Some(CNF)``` if formula is CNF and ```None``` else (unfortunately, it cannot build CNF from any formula for now, but this feature can appear in future).

## SAT Solver
```
use formula::cnf::SATSolver
```

This is a struct that contains only static member functions (so you do not need to define an instance of it). It provides ```SATSolver::solve(CNF)``` method, which returns ```None``` if there is no evaulation for CNF, and ```Some(CNF)``` - evaluation set else. The main algorithm is DPLL. Also, there is an option you can call some stages of it separately.
