use super::metadata;
use super::function;
use super::value::{ Evaluation, Value };

/// Tests.
#[cfg(test)]
mod tests;

/// A title formatting expression.
pub struct Expression<'a, T: metadata::Provider>
	where T: 'a {
	items: Vec<Item<'a, T>>,
}

/// An item of a formatting expression.
pub enum Item<'a, T: metadata::Provider>
	where T: 'a {
	/// Simple text.
	Text(String),
	/// Metadata tag.
	///
	/// Defined in the expression string between percentage signs: `%tag_name%`.
	Tag(String),
	/// Optional sub-expression.
	///
	/// Returns an empty string if none of the tags in the sub-expression was found.
	/// Defined in the expression string between square brackets: `[this is an optional sub-expression with a %tag%]`.
    OptionalExpr(Box<Expression<'a, T>>),
	/// A function call.
	///
	/// Defined in the expression string with a dollar sign and a comma-separated argument list between parentheses: `$function(arg1, arg2, ...)`.
	Function(FunctionCall<'a, T>),
}

/// A function call.
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
	pub fn apply(&self, metadata_provider: &T) -> Evaluation {
		let mut v: Vec<Evaluation> = Vec::new();
		for item in self.items.iter() {
			let evaluation : Evaluation = {
				match item {
					&Item::Text(ref text) => {
						let result_text = text.clone();
						// for plain text: the truth value is false
						Evaluation::new(Value::Text(result_text), false)
					},
					&Item::Tag(ref text) => {
						// check the tag in lowercase
						let tag_result = metadata_provider.tag_value(text.to_lowercase().as_str());
						if let Some(result_string) = tag_result {
							Evaluation::new(Value::Text(result_string), true)
						}
						else {
							Evaluation::new(Value::Unknown, false)
						}
					},
					&Item::OptionalExpr(ref expr) => {
						let expr_result = expr.apply(metadata_provider);
						match expr_result.truth() {
							true => expr_result,
							false => Evaluation::new(Value::Empty, false)
						}
					},
					&Item::Function(ref function_call) => {
						let function_res = function_call.evaluate(metadata_provider);
						match function_res {
							Ok(function_eval) => function_eval,
							Err(_) => Evaluation::new(Value::Empty, false),
						}
					},
				}
			};
			v.push(evaluation);
		}
		Evaluation::concatenate(&v[..])
	}
}

impl<'a, T: metadata::Provider> FunctionCall<'a, T> {
	pub fn new(function: &'a function::Function<T>, arguments: Vec<Box<Expression<'a, T>>>) -> FunctionCall<'a, T> {
		FunctionCall {
			function,
			arguments,
		}
	}
	fn evaluate(&self, metadata_provider: &T) -> Result<Evaluation, function::Error> {
		self.function.apply(&self.arguments[..], metadata_provider)
	}	
}
