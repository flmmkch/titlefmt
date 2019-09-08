use crate::expression::{Evaluation, Expression};
use crate::metadata;

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

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

/// Generic type for function trait objects.
pub type FunctionClosure<T> = dyn Fn(&[Box<Expression<T>>], &T) -> Result<Evaluation, Error>;

/// Definition of a function that can be used in expressions.
pub struct Function<T: metadata::Provider> {
    /// Closure used for applying the function.
    closure: Box<FunctionClosure<T>>,
    /// Name of the function.
    name: String,
}

/// Get the integer result for a sub-expression and return an Option<T> where T is the integer type. If no type is given, then i32 is assumed.
#[macro_export]
macro_rules! try_integer_result {
    ($expression: expr, $provider: expr, $type: ty) => {{
        let eval = $expression.apply($provider);
        let i_opt: Option<$type> = {
            match eval.value() {
                &Value::Integer(term) => Some(term as $type),
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

/// Get the integer result for a sub-expression or return an error.
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

/// Get the string result for a sub-expression or return an error.
#[macro_export]
macro_rules! expect_string_result {
    ($expression: expr, $provider: expr) => {{
        let eval = $expression.apply($provider);
        (eval.to_string(), eval.truth())
    }};
}

/// Make a new Function from a function, using the name of the argument provided as its name.
#[macro_export]
macro_rules! make_function_object {
    ($($function_part:ident)::*, $func_name:expr) => {
        Function::new(
            String::from($func_name),
            Box::new(
                |expressions,
                    provider|
                    -> Result<Evaluation, Error> { $($function_part)::*(expressions, provider) },
            ),
        )
    };
    ($($function_part:ident)::*) => {{
        let mut _function_name: &'static str;
        $(
            _function_name = stringify!($function_part);
        )*
        make_function_object!($($function_part)::*, _function_name)
    }};
}

/// Arithmetic functions
pub mod arithmetic;
/// Control flow functions
pub mod control_flow;
/// Boolean functions
pub mod logical;
// String functions
pub mod string;

/// Initialize a list of the standard functions defined in title formatting.
pub fn standard_functions<T: metadata::Provider>() -> impl Iterator<Item = Function<T>> {
    macro_rules! add_function {
        ($previous_iterator:expr, $($($argument:tt)::*),*) => {
            $previous_iterator.chain(Some(make_function_object!($($($argument)::*),*)))
        };
        ($($($argument:tt)::*),*) => {
            Some(make_function_object!($($($argument)::*),*)).into_iter()
        };
    }
    // arithmetic functions
    let result_iterator = add_function!(arithmetic::add::add);
    let result_iterator = add_function!(result_iterator, arithmetic::div::div);
    let result_iterator = add_function!(result_iterator, arithmetic::greater::greater);
    let result_iterator = add_function!(result_iterator, arithmetic::max::max);
    let result_iterator = add_function!(result_iterator, arithmetic::min::min);
    let result_iterator = add_function!(result_iterator, arithmetic::mod_::mod_, "mod");
    let result_iterator = add_function!(result_iterator, arithmetic::mul::mul);
    let result_iterator = add_function!(result_iterator, arithmetic::muldiv::muldiv);
    let result_iterator = add_function!(result_iterator, arithmetic::sub::sub);
    // logical boolean functions
    let result_iterator = add_function!(result_iterator, logical::and::and);
    let result_iterator = add_function!(result_iterator, logical::or::or);
    let result_iterator = add_function!(result_iterator, logical::not::not);
    let result_iterator = add_function!(result_iterator, logical::xor::xor);
    // control flow functions
    let result_iterator = add_function!(result_iterator, control_flow::if_::if_, "if");
    let result_iterator = add_function!(result_iterator, control_flow::if2::if2);
    let result_iterator = add_function!(result_iterator, control_flow::if3::if3);
    let result_iterator = add_function!(result_iterator, control_flow::ifequal::ifequal);
    let result_iterator = add_function!(result_iterator, control_flow::ifgreater::ifgreater);
    let result_iterator = add_function!(result_iterator, control_flow::iflonger::iflonger);
    let result_iterator = add_function!(result_iterator, control_flow::select::select);
    // string functions
    let result_iterator = add_function!(result_iterator, string::abbr::abbr);
    #[cfg(feature = "unicode-normalization")]
    let result_iterator = add_function!(result_iterator, string::ascii::ascii);
    let result_iterator = add_function!(result_iterator, string::caps::caps);
    let result_iterator = add_function!(result_iterator, string::caps2::caps2);
    let result_iterator = add_function!(result_iterator, string::left::left, "cut");
    let result_iterator = add_function!(result_iterator, string::directory::directory);
    let result_iterator = add_function!(result_iterator, string::directory_path::directory_path);
    let result_iterator = add_function!(result_iterator, string::ext::ext);
    let result_iterator = add_function!(result_iterator, string::filename::filename);
    let result_iterator = add_function!(result_iterator, string::hex::hex);
    let result_iterator = add_function!(result_iterator, string::insert::insert);
    let result_iterator = add_function!(result_iterator, string::left::left);
    let result_iterator = add_function!(result_iterator, string::right::right);
    let result_iterator = add_function!(result_iterator, string::substr::substr);
    result_iterator
}

impl<T: metadata::Provider> Function<T> {
    pub fn new(name_param: String, closure: Box<FunctionClosure<T>>) -> Function<T> {
        Function {
            closure,
            name: name_param,
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
