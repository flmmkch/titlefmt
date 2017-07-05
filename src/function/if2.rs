use super::*;
use super::value::Value;

fn if2<T: metadata::Provider>(provider: &T, expressions: &[Box<expression::Expression<T>>]) -> Result<Value, Error> {
    if expressions.len() != 2 {
        return Err(Error::TypeError);
    }
    let expr_value = expressions[0].apply(provider);
    match expr_value {
        Value::Empty | Value::Boolean(false) => Ok(expressions[1].apply(provider)),
        _ => Ok(expr_value),
    }
}

pub fn make_function_object<T: metadata::Provider>() -> super::Function<T> {
    Function {
        closure: Box::new(|provider: &T, expressions: &[Box<expression::Expression<T>>]| -> Result<Value, Error> { if2(provider, expressions) }),
        name: "IF2".to_owned(),
    }
}