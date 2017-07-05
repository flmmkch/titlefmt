use nom::{IResult, alphanumeric};
use std::str;
use std::string;
use nom::ErrorKind;
use super::metadata;
use super::expression;

#[derive(Debug)]
pub enum ParseError {
    NomError,
    Incomplete,
    UnicodeError(string::FromUtf8Error),
    Unknown,
    FunctionNotFound(String),
}

/// Parsing a string: first parse into intermediate objects (building module)
/// Then convert into real objects with the context
pub fn parse<'a, T: metadata::Provider>(string: &str, format_parser: &'a super::FormatParser<T>) -> Result<expression::Expression<'a, T>, ParseError> {
    let result = parse_expression(string.as_bytes());
    match result {
        IResult::Done(_, building_expr) => {
            let real_expr = build_expression(building_expr, &format_parser)?;
            Ok(real_expr)
        },
        IResult::Error(ErrorKind::Custom(err)) => Err(err),
        _ => Err(ParseError::Unknown),
    }
}

/// Build the expression from the building plans that have been parsed, using the formatters' information (such as functions)
fn build_expression<'a, T: metadata::Provider>(building_expr: building::Expression, format_parser: &'a super::FormatParser<T>) -> Result<expression::Expression<'a, T>, ParseError> {
    let mut real_items = Vec::new();
    for building_item in building_expr.items {
        let s : expression::Item<T> = {
            match building_item {
                building::Item::Text(v) => expression::Item::Text(v),
                building::Item::Tag(v) => expression::Item::Tag(v),
                building::Item::OptionalExpr(optional_building_expr) => {
                    let optional_real_expr = build_expression(*optional_building_expr, &format_parser)?;
                    expression::Item::OptionalExpr(Box::new(optional_real_expr))
                },
                building::Item::Function(building_function_call) => {
                    let function_call = {
                        let func = {
                            match format_parser.find_function(building_function_call.name.as_str()) {
                                Some(real_func) => real_func,
                                None => return Err(ParseError::FunctionNotFound(building_function_call.name)),
                            }
                        };
                        let mut real_args = Vec::new();
                        for building_arg in building_function_call.arguments {
                            let real_arg = build_expression(*building_arg, &format_parser)?;
                            real_args.push(Box::new(real_arg));
                        }
                        expression::FunctionCall::new(func, real_args)
                    };
                    expression::Item::Function(function_call)
                },
            }
        };
        real_items.push(s)
    }
    let expr = expression::Expression::new(real_items);
    Ok(expr)
}

mod building {
    /// A formatting expression being built
    pub struct Expression {
        pub items: Vec<Item>,
    }

    /// An item that is a composant of a formatting expression
    pub enum Item {
        /// Simple text
        Text(String),
        /// Metadata tag
        /// Signified in the definition between % signs: %tag_name%
        Tag(String),
        /// Optional sub-expression
        /// Returns an empty string if none of the tags in the sub-expression was found
        /// Signified in the definition between square brackets []
        OptionalExpr(Box<Expression>),
        /// A function call
        Function(FunctionCall),
    }

    pub struct FunctionCall {
        pub name: String,
        pub arguments: Vec<Box<Expression>>,
    }
}

fn make_escaped_text_item(string: &str) -> Result<building::Item, ParseError> {
    Ok(building::Item::Text(string.to_owned()))
}

named!(escaped_text<&[u8], building::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        alt!(
            // special rule: '' => turns to a single ' text
            value!(
                building::Item::Text("'".to_owned()),
                tag!("''")
                ) |
            // otherwise text enclosed with single quotes ' turn into normal text
            map_res!(
                map_res!(
                    delimited!(
                        tag!("'"),
                        take_until!("'"),
                        tag!("'")),
                    str::from_utf8
                ),
                make_escaped_text_item
            )
        )
    )
);

fn make_tag_item(string: &str) -> Result<building::Item, ParseError> {
    Ok(building::Item::Tag(string.to_owned()))
}

named!(item_tag<&[u8], building::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        map_res!(
        delimited!(
            tag!("%"),
            map_res!(alphanumeric, str::from_utf8),
            tag!("%")),
        make_tag_item
        )
    )
);

fn make_expression_box(expression: building::Expression) -> Result<Box<building::Expression>, ParseError> {
    Ok(Box::new(expression))
}

named!(optional_expression<&[u8], Box<building::Expression>>,
    map_res!(
        add_return_error!(
            ErrorKind::Custom(42),
            parse_expression
            ),
        make_expression_box
    )
);

fn make_optional_item(expression: Box<building::Expression>) -> Result<building::Item, ParseError> {
    Ok(building::Item::OptionalExpr(expression))
}

named!(item_optional<&[u8], building::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        map_res!(
            do_parse!(
                tag!("[") >>
                expr: optional_expression >>
                tag!("]") >>
                (expr)),
            make_optional_item
        )
    )
);

named!(parse_item<&[u8], building::Item, ParseError>,
    alt!(
        escaped_text |
        item_tag |
        item_optional
    )
);

fn flush_text(current_text: &mut Vec<u8>, items: &mut Vec<building::Item>) -> Result<(), ParseError> {
    if current_text.len() > 0 {
        let text_result = String::from_utf8(current_text.to_vec());
        match text_result {
            Ok(text) => {
                items.push(building::Item::Text(text));
                current_text.clear();
            },
            Err(e) => return Err(ParseError::UnicodeError(e)),
        }
    };
    Ok(())
}

macro_rules! flush_text {
    ($x:expr, $y:expr) => {
        if let Err(err) = flush_text($x, $y) {
            return IResult::Error(ErrorKind::Custom(err));
        };
    }
}

fn parse_expression(mut input: &[u8]) -> IResult<&[u8], building::Expression, ParseError> {
    let mut items: Vec<building::Item> = Vec::new();
    let mut current_text: Vec<u8> = Vec::new();
    while input.len() > 0 {
        // special characters
        match input[0] {
            b']' => break,
            _ => {
                let parse_result = parse_item(input);
                match parse_result {
                    IResult::Done(input_remaining, new_item) => {
                        input = input_remaining;
                        flush_text!(&mut current_text, &mut items);
                        items.push(new_item);
                    },
                    _ => {
                        current_text.push(input[0]);
                        input = &input[1..];
                    },
                }
            }
        }
    }
    flush_text!(&mut current_text, &mut items);
    let expression = building::Expression {
        items,
    };
    IResult::Done(input, expression)
}

