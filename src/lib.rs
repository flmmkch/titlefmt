//! `titleformat` is a title formatting expression library, enabling the processing and automatic formatting of media metadata.
//! 
//! # Example
//! ```
//! extern crate titleformat;
//!
//! use std::collections::HashMap;
//! use titleformat::{ metadata, Formatter };
//!
//! pub struct MetadataProvider<'a> {
//!     metadata_dict: HashMap<&'a str, &'a str>,
//! }
//!
//! impl<'a> metadata::Provider for MetadataProvider<'a> {
//!     fn tag_value(&self, key: &str) -> Option<String> {
//!         let entry = self.metadata_dict.get(key);
//!         if let Some(value) = entry {
//!             let s = value.to_string();
//!             Some(s)
//!         }
//!         else {
//!             None
//!         }
//!     }
//! }
//!
//! impl<'a> MetadataProvider<'a> {
//!     pub fn new(metadata_dict: HashMap<&'a str, &'a str>) -> MetadataProvider<'a> {
//!         MetadataProvider {
//!             metadata_dict,
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let formatter = Formatter::new();
//!     // tests with optional expressions
//!     {
//!         let expression = formatter.parser().parse("%tracknumber%.[ %artist% -] %title%").unwrap();
//!         {
//!             let test_metadata = {
//!                 let mut dict = HashMap::new();
//!                 dict.insert("tracknumber", "1");
//!                 dict.insert("title", "9th Symphony, 1. Allegro ma non troppo, un poco maestoso");
//!                 dict.insert("composer", "Ludwig van Beethoven");
//!                 MetadataProvider::new(dict)
//!             };
//!             let s = expression.apply(&test_metadata);
//!             assert_eq!("1. 9th Symphony, 1. Allegro ma non troppo, un poco maestoso", s.to_string().as_str());
//!         }
//!         {
//!             let test_metadata = {
//!                 let mut dict = HashMap::new();
//!                 dict.insert("tracknumber", "5");
//!                 dict.insert("title", "Always Crashing In The Same Car");
//!                 dict.insert("artist", "David Bowie");
//!                 MetadataProvider::new(dict)
//!             };
//!             let s = expression.apply(&test_metadata);
//!             assert_eq!("5. David Bowie - Always Crashing In The Same Car", s.to_string().as_str());
//!         }
//!     }
//! }
//! ```

#[macro_use]
extern crate nom;
extern crate num;
/// Metadata provider trait module.
pub mod metadata;
/// Expression module.
pub mod expression;
/// Basic implementation of a metadata provider using FFmpeg. Requires `features=ffmpeg`.
#[cfg(feature = "ffmpeg")]
pub mod ffmpeg_audio;

/// Values module.
pub mod value;
/// Functions module.
pub mod function;
/// Parser module
mod parser;

/// Tests module.
#[cfg(test)]
mod tests;

use std::collections::HashMap;

/// Title formatting context.
pub struct Formatter<T: metadata::Provider> {
    functions: Vec<Box<function::Function<T>>>,
}

/// Title formatting parser, spawned from a `Formatter`.
pub struct FormatParser<'a, T: metadata::Provider>
    where T: 'a {
    //formatter: &'a Formatter<T>,
    functions_map: HashMap<&'a str, &'a function::Function<T>>,
}

impl<T: metadata::Provider> Formatter<T> {
    /// Initialize a new formatting context with the standard functions.
    pub fn new() -> Formatter<T> {
        let functions = function::standard_functions();
        Formatter {
            functions,
        }
    }
    /// Add a new function to the title formatting context.
    pub fn add_function(&mut self, func: function::Function<T>) {
        self.functions.push(Box::new(func));
    }
    /// Initialize a title formatting expression parser using the context.
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
    fn find_function(&self, name: &str) -> Option<&'a function::Function<T>> {
        // we can't just return the result straight away because functions_map.get(..) returns Option<&&function::Function> and we want Option<&function::Function>
        match self.functions_map.get(name) {
            Some(func) => Some(func),
            None => None,
        }
    }
    /// Parse a title formatting string into an `Expression` that can be applied on metadata providers.
    pub fn parse<'b>(&'b self, string: &str) -> Result<expression::Expression<'a, T>, parser::ParseError> {
        Ok(parser::parse(string, &self)?)
    }
}
