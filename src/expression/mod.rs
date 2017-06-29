
use super::metadata::MetadataObject;

pub enum Item {
	Text(String),
	Tag(String),
    OptionalExpression(Box<Expression>),
}

pub struct Expression {
	items: Vec<Item>,
}

mod expression_parser;

impl Expression {
    pub fn parse(string: &str) -> Result<Expression, expression_parser::ParseError> {
        Ok(expression_parser::parse(string)?)
    }
	pub fn apply(&self, metadata: &MetadataObject) -> String {
		let mut s = String::new();
		for item in self.items.iter() {
			match item {
				&Item::Text(ref text) => s.push_str(text),
				&Item::Tag(ref text) => {
					let result = metadata.read_tag(text.to_lowercase().as_str());
					if let Some(result_string) = result {
						s.push_str(result_string.as_str())
					}
				},
				&Item::OptionalExpression(_) => (),
			}
		}
		s
	}
    pub fn definition(&self) -> String {
        let mut s = String::new();
		for item in self.items.iter() {
			match item {
				&Item::Text(ref text) => s.push_str(text),
				&Item::Tag(ref text) => {
						s.push_str("%");
						s.push_str(text.as_str());
						s.push_str("%");
					},
				&Item::OptionalExpression(_) => (),
			}
		}
        s
    }
}
