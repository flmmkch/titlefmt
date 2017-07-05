use super::*;

pub enum Error {
    ArgumentError,
    TypeError,
}

pub type FunctionClosure<T> = Fn(&T, &[Box<expression::Expression<T>>]) -> Result<value::Value, Error>;

pub struct Function<T: metadata::Provider> {
    closure: Box<FunctionClosure<T>>,
    name: String,
}

mod if2;

pub fn standard_functions<T: metadata::Provider>() -> Vec<Box<Function<T>>> {
    let mut s = Vec::new();
    s.push(Box::new(if2::make_function_object::<T>()));
    s
}

impl<T: metadata::Provider> Function<T> {
    pub fn apply(&self, provider: &T, arguments: &[Box<expression::Expression<T>>]) -> Result<value::Value, Error> {
        (self.closure)(&provider, &arguments)
    }
    pub fn name(&self, ) -> &str {
        self.name.as_str()
    }
}
