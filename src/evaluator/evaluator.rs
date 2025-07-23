use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::ast::Expression;
use crate::evaluator::{Environment, Callable, Value};

#[derive(Debug, PartialEq)]
pub enum EvalError {
    UndefinedVariable(String), // Attempt to access a variable that doesn't exist
    TypeError(String),         // Operation on incorrect type (e.g., adding a number to a string)
    WrongNumArgs(String),      // Function called with wrong number of arguments
    NotCallable(Value),        // Attempt to call a non-function value
    SpecialFormError(String),  // General error for malformed special forms
    DivisionByZero,            // Attempt to divide by zero
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: '{}'", name)
            }
            EvalError::TypeError(msg) => {
                write!(f, "Type error: {}", msg)
            }
            EvalError::WrongNumArgs(msg) => {
                write!(f, "Wrong number of arguments: {}", msg)
            }
            EvalError::NotCallable(value) => {
                write!(f, "Not a callable function: {:?}", value)
            }
            EvalError::SpecialFormError(msg) => {
                write!(f, "Special form error: {}", msg)
            }
            EvalError::DivisionByZero => {
                write!(f, "Division by zero")
            }
        }
    }
}

#[derive(Debug)]
pub struct Evaluator {
    pub global_env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            global_env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn evaluate(&self, expr: &Expression, env: Rc<RefCell<Environment>>) -> Result<Value, EvalError> {
        match expr {
            Expression::Number(n) => Ok(Value::Number(*n)),
            Expression::String(s) => Ok(Value::String(s.clone())),
            Expression::Boolean(b) => Ok(Value::Boolean(*b)),

            Expression::Identifier(name) => {
                env.borrow().get(name)
            }

            Expression::List(elements) => {
                if elements.is_empty() {
                    return Ok(Value::Nil);
                }

                let head = &elements[0];

                if let Expression::Identifier(op) = head {
                    match op.as_str() {
                        "if" => {
                            if elements.len() < 3 || elements.len() > 4 {
                                return Err(EvalError::WrongNumArgs(
                                    "if expects 2 or 3 arguments (condition then-expr [else-expr])"
                                        .to_string(),
                                ));
                            }
                            let condition = self.evaluate(&elements[1], env.clone())?;
                            if let Value::Boolean(true) = condition {
                                self.evaluate(&elements[2], env)
                            } else {
                                if elements.len() == 4 {
                                    self.evaluate(&elements[3], env)
                                } else {
                                    Ok(Value::Nil)
                                }
                            }
                        }
                        "let" => {
                            if elements.len() != 3 {
                                return Err(EvalError::WrongNumArgs(
                                    "let expects 2 arguments (variable value)".to_string(),
                                ));
                            }
                            let var_name_expr = &elements[1];
                            let value_expr = &elements[2];

                            let var_name = if let Expression::Identifier(name) = var_name_expr {
                                name
                            } else {
                                return Err(EvalError::TypeError(
                                    "let expects an identifier as variable name".to_string(),
                                ));
                            };

                            let value = self.evaluate(value_expr, env.clone())?;
                            env.borrow_mut().define(var_name.clone(), value);
                            Ok(Value::Nil)
                        }
                        "lambda" => {
                            if elements.len() < 3 {
                                return Err(EvalError::WrongNumArgs(
                                    "lambda expects at least (params) body".to_string(),
                                ));
                            }
                            let params_expr = &elements[1];
                            let body_exprs = elements[2..].to_vec();

                            let params = if let Expression::List(param_list) = params_expr {
                                param_list
                                    .iter()
                                    .map(|p_expr| {
                                        if let Expression::Identifier(p_name) = p_expr {
                                            Ok(p_name.clone())
                                        } else {
                                            Err(EvalError::TypeError(
                                                "lambda parameters must be identifiers".to_string(),
                                            ))
                                        }
                                    })
                                    .collect::<Result<Vec<String>, EvalError>>()?
                            } else {
                                return Err(EvalError::TypeError(
                                    "lambda parameters must be a list".to_string(),
                                ));
                            };

                            let captured_env = Rc::clone(&env);

                            Ok(Value::Function(Rc::new(Callable::Lambda {
                                params,
                                body: body_exprs,
                                env: captured_env,
                            })))
                        }
                        _ => {
                            self.apply_function_call(elements.to_vec(), env)
                        }
                    }
                } else {
                    self.apply_function_call(elements.to_vec(), env)
                }
            }
        }
    }

    fn eval_args(&self, args_exprs: &[Expression], env: Rc<RefCell<Environment>>) -> Result<Vec<Value>, EvalError> {
        args_exprs
            .iter()
            .map(|arg_expr| self.evaluate(arg_expr, env.clone()))
            .collect()
    }

    fn apply_function_call(&self, elements: Vec<Expression>, env: Rc<RefCell<Environment>>) -> Result<Value, EvalError> {
        let func_expr = &elements[0];
        let args_exprs = &elements[1..];

        let func_value = self.evaluate(func_expr, env.clone())?;
        let args_values = self.eval_args(args_exprs, env.clone())?;

        if let Value::Function(callable_rc) = func_value {
            let callable = &*callable_rc;

            match callable {
                Callable::Builtin(builtin_func) => builtin_func(args_values),
                Callable::Lambda { params, body, env: captured_env } => {
                    if args_values.len() != params.len() {
                        return Err(EvalError::WrongNumArgs(format!(
                            "Function expects {} arguments, but got {}",
                            params.len(),
                            args_values.len()
                        )));
                    }

                    let func_call_env = Rc::new(RefCell::new(
                        Environment::new_with_parent(Rc::clone(captured_env))
                    ));

                    for (param_name, arg_value) in params.iter().zip(args_values.into_iter()) {
                        func_call_env
                            .borrow_mut()
                            .define(param_name.clone(), arg_value);
                    }

                    let mut result = Value::Nil;
                    for expr in body {
                        result = self.evaluate(expr, func_call_env.clone())?;
                    }
                    Ok(result)
                }
            }
        } else {
            Err(EvalError::NotCallable(func_value))
        }
    }

    pub fn eval_program(&self, program: &[Expression]) -> Result<Value, EvalError> {
        let mut last_result = Value::Nil;
        let global_env = self.global_env.clone();

        for expr in program {
            last_result = self.evaluate(expr, global_env.clone())?;
        }
        Ok(last_result)
    }
}