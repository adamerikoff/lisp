// environment.rs
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::evaluator::{Value, Callable, EvalError};
use crate::evaluator::builtins;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            store: HashMap::new(),
            parent: None,
        };

        let builtins_to_register: &[(&str, fn(Vec<Value>) -> Result<Value, EvalError>)] = &[
            ("+", builtins::builtin_add),
            ("-", builtins::builtin_sub),
            ("*", builtins::builtin_mul),
            ("/", builtins::builtin_div),
            ("=", builtins::builtin_eq),
            ("!=", builtins::builtin_ne),
            (">", builtins::builtin_gt),
            ("<", builtins::builtin_lt),
            (">=", builtins::builtin_ge),
            ("<=", builtins::builtin_le),
            ("print", builtins::builtin_print),
        ];

        for (name, func) in builtins_to_register {
            env.define(name.to_string(), Value::Function(Rc::new(Callable::Builtin(*func))));
        }

        env
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, EvalError> {
        if let Some(value) = self.store.get(name) {
            Ok(value.clone())
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow().get(name)
        } else {
            Err(EvalError::UndefinedVariable(name.to_string()))
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    pub fn set(&mut self, name: String, value: Value) -> Result<(), EvalError> {
        if self.store.contains_key(&name) {
            self.store.insert(name, value);
            Ok(())
        } else if let Some(parent_env) = &self.parent {
            parent_env.borrow_mut().set(name, value)
        } else {
            Err(EvalError::UndefinedVariable(name))
        }
    }
}