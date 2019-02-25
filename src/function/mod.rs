use expression::{Evaluation, Expression};
use metadata;
use std;

/// Error encountered when applying a function.
#[derive(Debug)]
pub enum Error {
    ArgumentError,
    TypeError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ArgumentError => "Argument error",
            &Error::TypeError => "Type error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

/// Generic type for function trait objects.
pub type FunctionClosure<T> = Fn(&[Box<Expression<T>>], &T) -> Result<Evaluation, Error>;

/// Definition of a function that can be used in expressions.
pub struct Function<T: metadata::Provider> {
    /// Closure used for applying the function.
    closure: Box<FunctionClosure<T>>,
    /// Name of the function.
    name: String,
}

macro_rules! function_object_maker {
    ($func_name: ident) => {
        pub fn make_function_object<T: metadata::Provider>() -> Function<T> {
            Function::new(
                stringify!($func_name),
                Box::new(
                    |expressions: &[Box<Expression<T>>],
                     provider: &T|
                     -> Result<Evaluation, Error> { $func_name(expressions, provider) },
                ),
            )
        }
    };
}

#[macro_export]
macro_rules! try_integer_result {
    ($expression: expr, $provider: expr, $type: ty) => {{
        let eval = $expression.apply($provider);
        let i_opt: Option<$type> = {
            match eval.value() {
                &Value::Integer(term) => Some(term as $type),
                &Value::Double(term) => Some(term as $type),
                &Value::Text(ref s) => match s.parse::<$type>() {
                    Ok(term) => Some(term),
                    _ => None,
                },
                _ => None,
            }
        };
        if let Some(i) = i_opt {
            Some((i, eval.truth()))
        } else {
            None
        }
    }};
    ($expression: expr, $provider: expr) => {
        try_integer_result!($expression, $provider, i32)
    };
}

#[macro_export]
macro_rules! expect_integer_result {
    ($expression: expr, $provider: expr, $type: ty) => {
        match try_integer_result!($expression, $provider, $type) {
            Some(eval) => eval,
            None => return Err(Error::TypeError),
        }
    };
    ($expression: expr, $provider: expr) => {
        expect_integer_result!($expression, $provider, i32)
    };
}

#[macro_export]
macro_rules! expect_string_result {
    ($expression: expr, $provider: expr) => {{
        let eval = $expression.apply($provider);
        (eval.to_string(), eval.truth())
    }};
}

/// Arithmetic functions
mod arithmetic;
use self::arithmetic::*;
/// Boolean functions
mod logical;
use self::logical::*;
/// Control flow functions
mod control_flow;
use self::control_flow::*;
// String functions
mod string;
use self::string::*;

/// Initialize a list of the standard functions defined in title formatting.
pub fn standard_functions<T: metadata::Provider>() -> Vec<Box<Function<T>>> {
    let mut s = Vec::new();
    macro_rules! add_function {
        ($func_name: ident) => {
            s.push(Box::new($func_name::make_function_object::<T>()));
        };
    }
    // arithmetic functions
    add_function!(add);
    add_function!(div);
    add_function!(greater);
    add_function!(max);
    add_function!(min);
    s.push(Box::new(mod_::make_function_object::<T>()));
    add_function!(mul);
    add_function!(muldiv);
    add_function!(sub);
    // logical boolean functions
    add_function!(and);
    add_function!(or);
    add_function!(not);
    add_function!(xor);
    // control flow functions
    s.push(Box::new(if_::make_function_object::<T>()));
    add_function!(if2);
    add_function!(if3);
    add_function!(ifequal);
    add_function!(ifgreater);
    add_function!(iflonger);
    add_function!(select);
    // string functions
    add_function!(abbr);
    add_function!(caps);
    add_function!(caps2);
    add_function!(crlf);
    add_function!(cut);
    add_function!(directory);
    add_function!(directory_path);
    add_function!(ext);
    add_function!(filename);
    add_function!(hex);
    add_function!(insert);
    s
}

impl<T: metadata::Provider> Function<T> {
    pub fn new(name_param: &str, closure: Box<FunctionClosure<T>>) -> Function<T> {
        Function {
            closure,
            name: name_param.to_lowercase(),
        }
    }
    pub fn apply(
        &self,
        arguments: &[Box<Expression<T>>],
        provider: &T,
    ) -> Result<Evaluation, Error> {
        (self.closure)(&arguments, &provider)
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}
