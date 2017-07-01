
use super::metadata::MetadataObject;

pub enum Item {
	Text(String),
	Tag(String),
    OptionalExpr(Box<Expression>),
}

pub struct Expression {
	items: Vec<Item>,
}

mod expression_parser;

impl Expression {
    pub fn parse(string: &str) -> Result<Expression, expression_parser::ParseError> {
        Ok(expression_parser::parse(string)?)
    }
	fn apply_optional(&self, metadata: &MetadataObject) -> (String, u32) {
		let mut s = String::new();
		let mut tags_found : u32 = 0; 
		for item in self.items.iter() {
			match item {
				&Item::Text(ref text) => s.push_str(text),
				&Item::Tag(ref text) => {
					let result = metadata.read_tag(text.to_lowercase().as_str());
					if let Some(result_string) = result {
						s.push_str(result_string.as_str());
						if result_string.len() > 0 {
							tags_found += 1;
						}
					}
				},
				&Item::OptionalExpr(ref expr) => {
					let (expr_s, expr_tag) = expr.apply_optional(metadata);
					if expr_tag > 0 {
						s.push_str(expr_s.as_str());
						tags_found += expr_tag;
					}
				},
			}
		}
		(s, tags_found)
	}
	pub fn apply(&self, metadata: &MetadataObject) -> String {
		let (s, _) = self.apply_optional(metadata);
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
				&Item::OptionalExpr(ref expr) => {
					s.push_str("[");
					s.push_str(expr.definition().as_str());
					s.push_str("]");
				},
			}
		}
        s
    }
}
