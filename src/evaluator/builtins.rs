use super::{Value, EvalError};


fn check_num_args(func_name: &str, args: &[Value], expected: usize) -> Result<(), EvalError> {
    if args.len() != expected {
        Err(EvalError::WrongNumArgs(format!(
            "{} expects {} arguments, but got {}",
            func_name,
            expected,
            args.len()
        )))
    } else {
        Ok(())
    }
}

fn check_min_args(func_name: &str, args: &[Value], min_expected: usize) -> Result<(), EvalError> {
    if args.len() < min_expected {
        Err(EvalError::WrongNumArgs(format!(
            "{} expects at least {} arguments, but got {}",
            func_name,
            min_expected,
            args.len()
        )))
    } else {
        Ok(())
    }
}

fn get_num_arg(func_name: &str, arg: &Value) -> Result<f64, EvalError> {
    if let Value::Number(n) = arg {
        Ok(*n)
    } else {
        Err(EvalError::TypeError(format!("{} expects numbers", func_name)))
    }
}

fn get_two_num_args(func_name: &str, args: &[Value]) -> Result<(f64, f64), EvalError> {
    check_num_args(func_name, args, 2)?;
    let a = get_num_arg(func_name, &args[0])?;
    let b = get_num_arg(func_name, &args[1])?;
    Ok((a, b))
}

fn get_all_num_args(func_name: &str, args: Vec<Value>) -> Result<Vec<f64>, EvalError> {
    args.into_iter()
        .map(|arg| get_num_arg(func_name, &arg))
        .collect()
}

// Arithmetic functions
pub fn builtin_add(args: Vec<Value>) -> Result<Value, EvalError> {
    let numbers = get_all_num_args("+", args)?;
    Ok(Value::Number(numbers.iter().sum()))
}

pub fn builtin_sub(args: Vec<Value>) -> Result<Value, EvalError> {
    check_min_args("-", &args, 1)?;
    let numbers = get_all_num_args("-", args)?;
    if numbers.len() == 1 {
        Ok(Value::Number(-numbers[0])) // Unary minus
    } else {
        let first = numbers[0];
        let rest_sum: f64 = numbers.into_iter().skip(1).sum();
        Ok(Value::Number(first - rest_sum))
    }
}

pub fn builtin_mul(args: Vec<Value>) -> Result<Value, EvalError> {
    let numbers = get_all_num_args("*", args)?;
    Ok(Value::Number(numbers.iter().product()))
}

pub fn builtin_div(args: Vec<Value>) -> Result<Value, EvalError> {
    let (numerator, denominator) = get_two_num_args("/", &args)?;
    if denominator == 0.0 {
        return Err(EvalError::DivisionByZero);
    }
    Ok(Value::Number(numerator / denominator))
}

// Comparison functions
pub fn builtin_eq(args: Vec<Value>) -> Result<Value, EvalError> {
    check_num_args("=", &args, 2)?;
    Ok(Value::Boolean(args[0] == args[1]))
}

pub fn builtin_ne(args: Vec<Value>) -> Result<Value, EvalError> {
    check_num_args("!=", &args, 2)?;
    Ok(Value::Boolean(args[0] != args[1]))
}

pub fn builtin_gt(args: Vec<Value>) -> Result<Value, EvalError> {
    let (a, b) = get_two_num_args(">", &args)?;
    Ok(Value::Boolean(a > b))
}

pub fn builtin_lt(args: Vec<Value>) -> Result<Value, EvalError> {
    let (a, b) = get_two_num_args("<", &args)?;
    Ok(Value::Boolean(a < b))
}

pub fn builtin_ge(args: Vec<Value>) -> Result<Value, EvalError> {
    let (a, b) = get_two_num_args(">=", &args)?;
    Ok(Value::Boolean(a >= b))
}

pub fn builtin_le(args: Vec<Value>) -> Result<Value, EvalError> {
    let (a, b) = get_two_num_args("<=", &args)?;
    Ok(Value::Boolean(a <= b))
}

// Other built-ins
pub fn builtin_print(args: Vec<Value>) -> Result<Value, EvalError> {
    for (i, arg) in args.iter().enumerate() {
        print!("{}", arg);
        if i < args.len() - 1 {
            print!(" ");
        }
    }
    println!();
    Ok(Value::Nil)
}