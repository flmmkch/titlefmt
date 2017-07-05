use std::fmt;

#[derive(Clone)]
pub enum Value {
	Text(String),
	Integer(i32),
	Double(f64),
	Boolean(bool),
	Empty,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&Value::Text(ref v) => write!(f, "{}", v),
			&Value::Integer(ref v) => write!(f, "{}", v),
			&Value::Double(ref v) => write!(f, "{}", v),
			&Value::Boolean(ref v) => write!(f, "{}", v),
			&Value::Empty => write!(f, "?"),
		}
    }
}

impl Value {
	pub fn concatenate(values: &[Value]) -> Value {
		match values.len() {
			0 => Value::Empty,
			1 => values[0].clone(),
			_ => {
				let mut string_total = String::new();
				for value in values.iter() {
					string_total.push_str(value.to_string().as_str())
				}
				Value::Text(string_total)
			},
		}
	}
}