use super::FormatParser;
use function;
use metadata;

/// Title formatting context.
pub struct Formatter<T: metadata::Provider> {
    functions: Vec<function::Function<T>>,
}

impl<T: metadata::Provider> Formatter<T> {
    /// Initialize a new formatting context with the standard functions.
    pub fn new() -> Formatter<T> {
        let functions = function::standard_functions().collect();
        Formatter { functions }
    }
    /// Initialize a new formatting context without any function.
    pub fn new_empty() -> Formatter<T> {
        Formatter {
            functions: Vec::new(),
        }
    }
    pub fn functions(&self) -> &Vec<function::Function<T>> {
        &self.functions
    }
    /// Add a new function to the title formatting context.
    pub fn add_function(&mut self, func: function::Function<T>) {
        self.functions.push(func);
    }
    /// Initialize a title formatting expression parser using the context.
    pub fn parser(&self) -> FormatParser<T> {
        FormatParser::new(self)
    }
}
