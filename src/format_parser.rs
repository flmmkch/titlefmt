use super::parser;
use super::Formatter;
use expression;
use function;
use metadata;

use std::collections::HashMap;

/// Title formatting parser, spawned from a `Formatter`.
pub struct FormatParser<'a, T: metadata::Provider>
where
    T: 'a,
{
    //formatter: &'a Formatter<T>,
    functions_map: HashMap<&'a str, &'a function::Function<T>>,
}

impl<'a, T: metadata::Provider> FormatParser<'a, T> {
    pub fn new(formatter: &'a Formatter<T>) -> FormatParser<'a, T> {
        let mut functions_map = HashMap::new();
        for func in formatter.functions().iter() {
            functions_map.insert(func.name(), func);
        }
        FormatParser {
            //    formatter,
            functions_map,
        }
    }
    pub fn find_function(&self, name: &str) -> Option<&'a function::Function<T>> {
        // we can't just return the result straight away because functions_map.get(..) returns Option<&&function::Function> and we want Option<&function::Function>
        match self.functions_map.get(name) {
            Some(func) => Some(func),
            None => None,
        }
    }
    /// Parse a title formatting string into an `Expression` that can be applied on metadata providers.
    pub fn parse<'b>(
        &'b self,
        string: &str,
    ) -> Result<expression::Expression<'a, T>, parser::ParseError> {
        Ok(parser::parse(string, &self)?)
    }
}
