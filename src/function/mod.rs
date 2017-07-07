use super::*;

use num;

/// Error encountered when applying a function.
pub enum Error {
    ArgumentError,
    TypeError,
}

/// Generic type for function trait objects.
pub type FunctionClosure<T> = Fn(&T, &[Box<expression::Expression<T>>]) -> Result<value::Value, Error>;

/// Definition of a function that can be used in expressions.
pub struct Function<T: metadata::Provider> {
    /// Closure used for applying the function.
    closure: Box<FunctionClosure<T>>,
    /// Name of the function.
    name: String,
}

macro_rules! function_object_maker {
    ($func_name: ident) => {
        pub fn make_function_object<T: metadata::Provider>() -> super::Function<T> {
            Function::new(
                stringify!($func_name),
                Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { $func_name(provider, expressions) })
            )
        }
    }
}

mod add;
mod div;
mod greater;
mod max;
mod min;
mod mod_;
mod mul;
mod muldiv;
mod sub;
mod if_;
mod if2;

/// Initialize a list of the standard functions defined in title formatting.
pub fn standard_functions<T: metadata::Provider>() -> Vec<Box<Function<T>>> {
    let mut s = Vec::new();
    macro_rules! add_function {
        ($func_name: ident) => {
            s.push(Box::new($func_name::make_function_object::<T>()));
        }
    }
    add_function!(add);
    add_function!(div);
    add_function!(greater);
    add_function!(max);
    add_function!(min);
    s.push(Box::new(mod_::make_function_object::<T>()));
    add_function!(mul);
    add_function!(muldiv);
    add_function!(sub);
    s.push(Box::new(if_::make_function_object::<T>()));
    add_function!(if2);
    s
}

impl<T: metadata::Provider> Function<T> {
    pub fn new(name_param: &str, closure: Box<FunctionClosure<T>>) -> Function<T> {
        Function {
            closure,
            name: name_param.to_lowercase(),
        }
    }
    pub fn apply(&self, provider: &T, arguments: &[Box<expression::Expression<T>>]) -> Result<value::Value, Error> {
        (self.closure)(&provider, &arguments)
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

fn expect_integer_result<V, T: metadata::Provider>(expr: &expression::Expression<T>, provider: &T) -> Result<V, Error>
    where V: std::str::FromStr + num::FromPrimitive {
    match expr.apply(provider) {
        super::value::Value::Integer(term) => {
            match V::from_i32(term) {
                Some(v) => Ok(v),
                _ => Err(Error::TypeError),
            }
        },
        super::value::Value::Double(term) => {
            match V::from_f64(term) {
                Some(v) => Ok(v),
                _ => Err(Error::TypeError),
            }
        },
        super::value::Value::Text(s) => {
            match s.parse::<V>() {
                Ok(term) => Ok(term),
                _ => Err(Error::TypeError),
            }
        }
        _ => Err(Error::TypeError),
    }
}
