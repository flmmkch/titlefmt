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
//!             assert_eq!("01. 9th Symphony, 1. Allegro ma non troppo, un poco maestoso", s.to_string().as_str());
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
//!             assert_eq!("05. David Bowie - Always Crashing In The Same Car", s.to_string().as_str());
//!         }
//!     }
//! }
//! ```

#[macro_use]
extern crate nom;
/// Metadata provider trait module.
pub mod metadata;
/// Expression module.
pub mod expression;
/// Basic implementation of a metadata provider using FFmpeg. Requires `features=ffmpeg`.
#[cfg(feature = "ffmpeg")]
pub mod ffmpeg_audio;

/// Functions module.
#[macro_use]
pub mod function;
/// Parser module
mod parser;

/// Tests module.
#[cfg(test)]
mod test;

/// Formatter module.
mod formatter;
pub use formatter::Formatter;

/// FormatParser module.
mod format_parser;
pub use format_parser::FormatParser;
