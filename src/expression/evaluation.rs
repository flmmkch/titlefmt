use super::Value;
use std::fmt;

/// Result of a a complete or partial evaluation of a title formatting expression.
/// An Evaluation holds a result value, as well as a truth boolean used for sub-expressions between square brackets [].
/// If a sub-expression receives true as a truth value, then it will appear in the result. If the truth value is to false, then it will be hidden.
#[derive(Clone)]
pub struct Evaluation {
    value: Value,
    truth: bool,
}

impl Evaluation {
    pub fn new(value: Value, truth: bool) -> Evaluation {
        Evaluation { value, truth }
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
            }
        }
    }
}

impl fmt::Display for Evaluation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
