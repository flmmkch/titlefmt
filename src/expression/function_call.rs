use ::{ metadata, function };
use super::{ Expression, Evaluation };

/// A function call.
pub struct FunctionCall<'a, T: metadata::Provider>
	where T: 'a {
	function: &'a function::Function<T>,
	arguments: Vec<Box<Expression<'a, T>>>,
}

impl<'a, T: metadata::Provider> FunctionCall<'a, T> {
	pub fn new(function: &'a function::Function<T>, arguments: Vec<Box<Expression<'a, T>>>) -> FunctionCall<'a, T> {
		FunctionCall {
			function,
			arguments,
		}
	}
	pub fn evaluate(&self, metadata_provider: &T) -> Result<Evaluation, function::Error> {
		self.function.apply(&self.arguments[..], metadata_provider)
	}	
}
