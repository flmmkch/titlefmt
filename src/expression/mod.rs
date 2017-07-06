use super::metadata;
use super::function;
use super::value::Value;

/// Tests
#[cfg(test)]
mod tests;

/// A formatting expression
pub struct Expression<'a, T: metadata::Provider>
	where T: 'a {
	items: Vec<Item<'a, T>>,
}

/// An item that is a composant of a formatting expression
pub enum Item<'a, T: metadata::Provider>
	where T: 'a {
	/// Simple text
	Text(String),
	/// Metadata tag
	/// Signified in the definition between % signs: %tag_name%
	Tag(String),
	/// Optional sub-expression
	/// Returns an empty string if none of the tags in the sub-expression was found
	/// Signified in the definition between square brackets []
    OptionalExpr(Box<Expression<'a, T>>),
	/// A function call
	Function(FunctionCall<'a, T>),
}

pub struct FunctionCall<'a, T: metadata::Provider>
	where T: 'a {
	function: &'a function::Function<T>,
	arguments: Vec<Box<Expression<'a, T>>>,
}

impl<'a, T: metadata::Provider> Expression<'a, T> {
	pub fn new(items: Vec<Item<'a, T>>) -> Expression<'a, T> {
		Expression {
			items: items,
		}
	}
	pub fn apply(&self, metadata_provider: &T) -> Value {
		let (s, _) = self.apply_valued(metadata_provider);
		s
	}
	pub fn apply_valued(&self, metadata_provider: &T) -> (Value, u32) {
		let mut v: Vec<Value> = Vec::new();
		let mut tags_found : u32 = 0;
		for item in self.items.iter() {
			match item {
				&Item::Text(ref text) => v.push(Value::Text(text.clone())),
				&Item::Tag(ref text) => {
					let result = metadata_provider.tag_value(text.to_lowercase().as_str());
					if let Some(result_string) = result {
						if result_string.len() > 0 {
							tags_found += 1;
							v.push(Value::Text(result_string));
						}
					}
					else {
						v.push(Value::Empty);
					}
				},
				&Item::OptionalExpr(ref expr) => {
					let (expr_v, expr_tag) = expr.apply_valued(metadata_provider);
					if expr_tag > 0 {
						v.push(expr_v);
						tags_found += expr_tag;
					}
				},
				&Item::Function(ref function_call) => {
					let function_res = function_call.evaluate(metadata_provider);
					match function_res {
						Ok(function_v) => v.push(function_v),
						Err(_) => return (Value::Empty, 0),
					}
				},
			}
		}
		(Value::concatenate(&v[..]), tags_found)
	}
}

impl<'a, T: metadata::Provider> FunctionCall<'a, T> {
	pub fn new(function: &'a function::Function<T>, arguments: Vec<Box<Expression<'a, T>>>) -> FunctionCall<'a, T> {
		FunctionCall {
			function,
			arguments,
		}
	}
	fn evaluate(&self, metadata_provider: &T) -> Result<Value, function::Error> {
		self.function.apply(metadata_provider, &self.arguments[..])
	}	
}
