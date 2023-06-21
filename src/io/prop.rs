use std::fmt::{Debug};

use crate::p::cnf::{CNF, Literal};

pub struct PropositionalFormula {
    formula: String,
    tree: Option<ComputationTree>,
}

#[derive(Clone)]
struct ComputationTree {
    operation: String,
    children: Vec<ComputationTree>,
    value: Option<i32>,
}

impl Debug for ComputationTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl ComputationTree {
    fn fmt_with_indent(&self, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
        let str_indent = " ".repeat(indent);
        if self.value.is_some() {
            return write!(f, "{}- {}\n", str_indent, self.value.unwrap())
        } 
        write!(f, "{}{}: [\n", str_indent, self.operation)?;
        for child in &self.children {
            child.fmt_with_indent(f, indent + 2)?;
        }
        write!(f, "{}],\n", str_indent)
    }
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
            tree: None,
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

    fn get_other_op_children(&mut self, tr: &ComputationTree, operation: &str) -> Vec<ComputationTree> {
        let mut children = Vec::new();
        for child in tr.children.iter() {
            if child.operation != operation {
                children.push(self.reduce_tree_height(&child));
            } else {
                children.append(&mut self.get_other_op_children(&child, operation));
            }
        }
        children
    }

    fn reduce_tree_height(&mut self, tr: &ComputationTree) -> ComputationTree {
        if tr.value.is_some() {
            return tr.clone();
        }
        let operation = tr.operation.clone();
        ComputationTree { 
            operation: operation.clone(), 
            children: self.get_other_op_children(&tr, &operation), 
            value: None 
        }
    }

    fn build_computation_tree(&mut self, tokens: Vec<String>) -> ComputationTree {
        let mut stack = Vec::new();
        let mut operations = Vec::<String>::new();

        for token in tokens {
            if token == "(" {
                stack.push(Self::make_service_node(&token));
            } else if token == ")" {
                let mut found_opening_parenthesis = false;
                while !found_opening_parenthesis {
                    let first = stack.last().unwrap().clone();
                    stack.pop();
                    if stack.last().unwrap().operation == "(".to_string() {
                        stack.pop();
                        stack.push(first);
                        break;
                    }
                    stack.pop();
                    let operation = operations.pop().unwrap();
                    let binary = self.is_binary_operation(operation.as_str());
                    if !binary {
                        stack.push(Self::make_unary_operation_node(operation, first));
                    } else {
                        let second = stack.last().unwrap().clone();
                        stack.pop();
                        if stack.last().unwrap().operation == "(".to_string() {
                            found_opening_parenthesis = true;
                            stack.pop();
                        }
                        stack.push(Self::make_binary_operation_node(
                            operation,
                            first,
                            second
                        ));
                    }
                }
            } else if self.is_binary_operation(&token) || self.is_unary_operation(&token) {
                stack.push(Self::make_service_node("_"));
                operations.push(token);
            } else {
                stack.push(Self::make_variable_node(token.parse::<i32>().unwrap()));
            }
        }
        stack.pop().unwrap()
    }

    pub fn parse(&mut self) {
        self.formula = ["(", &self.formula, ")"].concat();

        let tokens = self.tokenize();
        let mut tr = self.build_computation_tree(tokens);
        tr = self.reduce_tree_height(&tr);
        self.tree = Some(tr);
    }

    pub fn get_cnf(&mut self) -> Option<CNF> {
        if self.tree.is_none() {
            return None;
        }

        let mut cnf = CNF::new();
        if self.tree.as_ref().unwrap().operation != "and" {
            return None;
        }

        for child in self.tree.as_ref().unwrap().children.iter() {
            if child.operation != "or" {
                return None;
            }
            let mut clause = Vec::new();
            for grandchild in child.children.iter() {
                if grandchild.operation == "not" {
                    if grandchild.children.len() != 1 || grandchild.children[0].value.is_none() {
                        return None;
                    }

                    clause.push(Literal {
                        var: grandchild.children[0].value.unwrap() as usize,
                        sign: true,
                    });
                } else {
                    if grandchild.value.is_none() {
                        return None;
                    }

                    clause.push(Literal { 
                        var: grandchild.value.unwrap() as usize, 
                        sign: false,
                    });
                }
            }
            cnf.add_clause(clause);
        }
        Some(cnf)
    }
}