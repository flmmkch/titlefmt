#[macro_use]
extern crate nom;
pub mod metadata;
pub mod expression;
#[cfg(feature = "ffmpeg")]
pub mod ffmpeg_audio;

/// Values module
pub mod value;
/// Functions module
#[macro_use]
pub mod function;

mod parser;

/// Tests
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub struct Formatter<T: metadata::Provider> {
    functions: Vec<Box<function::Function<T>>>,
}

pub struct FormatParser<'a, T: metadata::Provider>
    where T: 'a {
    //formatter: &'a Formatter<T>,
    functions_map: HashMap<&'a str, &'a function::Function<T>>,
}

impl<T: metadata::Provider> Formatter<T> {
    pub fn new() -> Formatter<T> {
        let functions = function::standard_functions();
        Formatter {
            functions,
        }
    }
    pub fn add_function(&mut self, func: function::Function<T>) {
        self.functions.push(Box::new(func));
    }
    pub fn parser(&self) -> FormatParser<T> {
        FormatParser::new(self)
    }
}

impl<'a, T: metadata::Provider> FormatParser<'a, T> {
    fn new(formatter: &'a Formatter<T>) -> FormatParser<'a, T> {
        let mut functions_map = HashMap::new();
        for func in formatter.functions.iter() {
            functions_map.insert(func.name(), func.as_ref());
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
    pub fn parse<'b>(&'b self, string: &str) -> Result<expression::Expression<'a, T>, parser::ParseError> {
        Ok(parser::parse(string, &self)?)
    }
}
