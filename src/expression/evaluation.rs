use std::fmt;
use super::Value;

/// Result of a a complete or partial evaluation of a title formatting expression.
#[derive(Clone)]
pub struct Evaluation {
	value: Value,
	truth: bool,
}

impl Evaluation {
	pub fn new(value: Value, truth: bool) -> Evaluation {
		Evaluation {
			value,
			truth,
		}
	}
	pub fn value(&self) -> &Value {
		&self.value
	}
	pub fn truth(&self) -> bool {
		self.truth
	}
	/// Concatenate an array of evaluations into one.
	pub fn concatenate(evaluations: &[Evaluation]) -> Evaluation {
		match evaluations.len() {
			0 => Evaluation::new(Value::Empty, false),
			1 => evaluations[0].clone(),
			_ => {
				let mut string_total = String::new();
				let mut truth_total = false;
				for eval in evaluations.iter() {
					string_total.push_str(eval.to_string().as_str());
					truth_total |= eval.truth;
				}
				Evaluation::new(Value::Text(string_total), truth_total)
			},
		}
	}
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.value)
    }
}
