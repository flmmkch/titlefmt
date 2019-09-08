use super::{Evaluation, Item, Value};
use crate::metadata;

/// A title formatting expression.
pub struct Expression<'a, T: metadata::Provider>
where
    T: 'a,
{
    items: Vec<Item<'a, T>>,
}

impl<'a, T: metadata::Provider> Expression<'a, T> {
    pub fn new(items: Vec<Item<'a, T>>) -> Expression<'a, T> {
        Expression { items }
    }
    fn evaluate_tag(given_tag: &str, metadata_provider: &T) -> Option<String> {
        let lowercase_tag = given_tag.to_lowercase();
        fn format_tracknumber(given_tracknumber_tag: Option<String>) -> Option<String> {
            match given_tracknumber_tag {
                Some(s) => {
                    if s.len() > 1 {
                        Some(s)
                    } else {
                        // parse to an unsigned integer
                        if let Ok(v) = s.parse::<u32>() {
                            Some(format!("{:02}", v))
                        } else {
                            Some(s)
                        }
                    }
                }
                None => None,
            }
        }
        // matching for special tags
        match lowercase_tag.as_str() {
            // track
            "track" => format_tracknumber(metadata_provider.tag_value("track")),
            // tracknumber
            "tracknumber" => {
                match format_tracknumber(metadata_provider.tag_value("tracknumber")) {
                    Some(string) => Some(string),
                    None => {
                        // try the TRACK tag if tracknumber didn't work
                        format_tracknumber(metadata_provider.tag_value("track"))
                    }
                }
            }
            // use a space to get the number without the leading zero
            "track number" => {
                match metadata_provider.tag_value("tracknumber") {
                    Some(string) => Some(string),
                    None => {
                        // try the TRACK field if tracknumber didn't work
                        metadata_provider.tag_value("track")
                    }
                }
            }
            "album artist" | "albumartist" | "album_artist" => {
                let mut test_try = metadata_provider.tag_value(&lowercase_tag);
                if (test_try == None) && (lowercase_tag != "album artist") {
                    test_try = metadata_provider.tag_value("album artist");
                }
                if (test_try == None) && (lowercase_tag != "albumartist") {
                    test_try = metadata_provider.tag_value("albumartist");
                }
                if (test_try == None) && (lowercase_tag != "album_artist") {
                    test_try = metadata_provider.tag_value("album_artist");
                }
                test_try
            }
            // any other tag
            _ => metadata_provider.tag_value(&lowercase_tag),
        }
    }
    fn evaluate_item(item: &Item<T>, metadata_provider: &T) -> Evaluation {
        match item {
            &Item::Text(ref text) => {
                let result_text = text.clone();
                // for plain text: the truth value is false
                Evaluation::new(Value::Text(result_text), false)
            }
            &Item::Tag(ref text) => {
                // check the tag in lowercase
                let tag_result = Expression::evaluate_tag(&text, metadata_provider);
                if let Some(result_string) = tag_result {
                    Evaluation::new(Value::Text(result_string), true)
                } else {
                    Evaluation::new(Value::Unknown, false)
                }
            }
            &Item::OptionalExpr(ref expr) => {
                let expr_result = expr.apply(metadata_provider);
                match expr_result.truth() {
                    true => expr_result,
                    false => Evaluation::new(Value::Empty, false),
                }
            }
            &Item::Function(ref function_call) => {
                let function_res = function_call.evaluate(metadata_provider);
                match function_res {
                    Ok(function_eval) => function_eval,
                    Err(_) => Evaluation::new(Value::Empty, false),
                }
            }
        }
    }
    /// Evaluate the expression for a metadata provider object
    pub fn apply(&self, metadata_provider: &T) -> Evaluation {
        let mut v: Vec<Evaluation> = Vec::new();
        for item in self.items.iter() {
            let evaluation = Expression::evaluate_item(&item, metadata_provider);
            v.push(evaluation);
        }
        Evaluation::concatenate(&v[..])
    }
}
