use std::borrow::Cow;
use std::fmt;

/// A value corresponding to a complete or partial evaluation of a title formatting expression.
#[derive(Clone)]
pub enum Value {
    Text(String),
    Integer(i32),
    Unknown,
    Empty,
}

impl Value {
    pub fn to_string<'a>(&'a self) -> Cow<'a, str> {
        match self {
            &Value::Text(ref v) => Cow::Borrowed(v),
            &Value::Integer(ref v) => Cow::Owned(v.to_string()),
            &Value::Empty => Cow::Borrowed(""),
            &Value::Unknown => Cow::Borrowed("?"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Text(ref v) => write!(f, "{}", v),
            &Value::Integer(ref v) => write!(f, "{}", v),
            &Value::Empty => write!(f, ""),
            &Value::Unknown => write!(f, "?"),
        }
    }
}
