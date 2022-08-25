pub struct PropositionalFormula {
    formula: String,
}

#[derive(Clone, Debug)]
struct ComputationTree {
    operation: String,
    children: Vec<ComputationTree>,
    value: Option<i32>,
}

impl PropositionalFormula {
    const BINARY_OPERATIONS: [&'static str; 4] = [
        "and",
        "or",
        "implies",
        "xor"
    ];
    const UNARY_OPERATIONS: [&'static str; 1] = [
        "not"
    ];

    pub fn new(formula: String) -> PropositionalFormula {
        PropositionalFormula {
            formula: formula,
        }
    }

    fn make_variable_node(var: i32) -> ComputationTree {
        ComputationTree {
            operation: String::new(),
            children: Vec::new(),
            value: Some(var),
        }
    }

    fn make_service_node(operation: &str) -> ComputationTree {
        ComputationTree {
            operation: String::from(operation),
            children: Vec::new(),
            value: None,
        }
    }
    
    fn get_operation_from_alias(operation: &str) -> &str {
        // TODO: Add multiple aliases for operations
        operation
    }

    fn is_binary_operation(&self, operation: &str) -> bool {
        for op in PropositionalFormula::BINARY_OPERATIONS.iter() {
            if op.contains(&operation) {
                return true;
            }
        }
        return false;
    }

    fn is_unary_operation(&self, operation: &str) -> bool {
        for op in PropositionalFormula::UNARY_OPERATIONS.iter() {
            if op.contains(&operation) {
                return true;
            }
        }
        return false;
    }

    fn make_binary_operation_node(operation: String, left: ComputationTree, right: ComputationTree) -> ComputationTree {
        ComputationTree {
            operation: operation,
            children: vec![left, right],
            value: None,
        }
    }

    fn make_unary_operation_node(operation: String, child: ComputationTree) -> ComputationTree {
        ComputationTree {
            operation: operation,
            children: vec![child],
            value: None,
        }
    }

    fn tokenize(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut token = String::new();
        
        for c in self.formula.chars() {
            match c {
                ' ' => {
                    if token.len() > 0 {
                        tokens.push(token);
                        token = String::new();
                    }
                }
                '(' => {
                    if token.len() > 0 {
                        tokens.push(token);
                        token = String::new();
                    }
                    tokens.push(String::from("("));
                }
                ')' => {
                    if token.len() > 0 {
                        tokens.push(token);
                        token = String::new();
                    }
                    tokens.push(String::from(")"));
                }
                _ => {
                    token.push(c);
                }
            }
        }
        if token.len() > 0 {
            tokens.push(token);
        }
        tokens
    }

    pub fn parse(&self) {
        let mut stack = Vec::new();
        let mut operations = Vec::<String>::new();
        let mut binary = false;
        let tokens = self.tokenize();

        println!("Tokens: {:?}", tokens);

        for token in tokens {
            println!("Token: {}", token);
            if token == "(" {
                stack.push(Self::make_service_node(&token));
            } else if token == ")" {
                println!("Operations stack: {:?}", operations);
                println!("Tree stack: {:?}", stack);
                let mut found_opening_parenthesis = false;
                while !found_opening_parenthesis {
                    let first = stack.last().unwrap().clone();
                    println!("Determined first: {:?}", first);
                    stack.pop();
                    if !binary {
                        println!("Unary operation, so pushing onto val stack");
                        stack.push(Self::make_unary_operation_node(operations.last().unwrap().clone(), first));
                    } else {
                        let second = stack.last().unwrap().clone();
                        println!("Deteriming second {:?}", second);
                        stack.pop();
                        if (stack.last().unwrap().operation == "(".to_string()) {
                            println!("Found opening parenthesis");
                            found_opening_parenthesis = true;
                            stack.pop();
                        }
                        stack.push(Self::make_binary_operation_node(
                            operations.last().unwrap().clone(),
                            first,
                            second
                        ));
                    }
                }
                println!("Stack state after closing bracket: {:?}", stack);
            } else if self.is_binary_operation(&token) {
                binary = true;
                operations.push(token);
            } else if self.is_unary_operation(&token) {
                operations.push(token);
            } else {
                stack.push(Self::make_variable_node(token.parse::<i32>().unwrap()));
            }
        }

    } 

}