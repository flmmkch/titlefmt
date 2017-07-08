use std::fmt;

/// A value corresponding to a complete or partial evaluation of a title formatting expression.
#[derive(Clone)]
pub enum Value {
	Text(String),
	Integer(i32),
	Double(f64),
	Unknown,
	Empty,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Value::Text(ref v) => write!(f, "{}", v),
			&Value::Integer(ref v) => write!(f, "{}", v),
			&Value::Double(ref v) => write!(f, "{}", v),
			&Value::Empty => write!(f, ""),
			&Value::Unknown => write!(f, "?"),
		}
    }
}
