use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::ast::Expression;
use crate::evaluator::{Environment, EvalError};

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Builtin(fn(Vec<Value>) -> Result<Value, EvalError>),
    Lambda {
        params: Vec<String>,
        body: Vec<Expression>,
        env: Rc<RefCell<Environment>>,
    },
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Callable::Builtin(_) => write!(f, "#<builtin-function>"),
            Callable::Lambda { params, .. } => write!(f, "#<lambda ({})>", params.join(" ")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),            // Floating-point numbers (e.g., 10, 3.14)
    String(String),         // Text strings (e.g., "hello world")
    Boolean(bool),          // Boolean values (true or false)
    Nil,                    // Represents Lisp's 'null' or 'void' value
    Function(Rc<Callable>), // A callable function (built-in or lambda)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Function(func) => write!(f, "{}", func),
        }
    }
}
