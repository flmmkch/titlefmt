use nom::{IResult, alphanumeric, is_alphanumeric, is_space};
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
    StrUnicodeError(str::Utf8Error),
    Unknown,
    FunctionNotFound(String),
}

/// Parsing a string: first parse into intermediate objects (building module)
/// Then convert into real objects with the context
pub fn parse<'a, 'b, T: metadata::Provider>(string: &str, format_parser: &super::FormatParser<'a, T>) -> Result<expression::Expression<'b, T>, ParseError>
    where 'a: 'b {
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
fn build_expression<'a, 'b, T: metadata::Provider>(building_expr: building::Expression, format_parser: &super::FormatParser<'a, T>) -> Result<expression::Expression<'b, T>, ParseError>
    where 'a: 'b {
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
                            let lowercase_func_name = building_function_call.name.to_lowercase();
                            match format_parser.find_function(lowercase_func_name.as_str()) {
                                Some(real_func) => real_func,
                                None => return Err(ParseError::FunctionNotFound(building_function_call.name)),
                            }
                        };
                        let mut real_args = Vec::new();
                        for building_arg in building_function_call.arguments {
                            let real_arg = build_expression(building_arg, &format_parser)?;
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
    #[derive(Debug)]
    pub struct Expression {
        pub items: Vec<Item>,
    }

    /// An item that is a composant of a formatting expression
    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct FunctionCall {
        pub name: String,
        pub arguments: Vec<Expression>,
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

fn optional_expression_expr(input: &[u8]) -> IResult<&[u8], building::Expression, ParseError> {
    limited_expression_parser(input, &[b']'])
}

named!(optional_expression<&[u8], Box<building::Expression>>,
    map_res!(
        add_return_error!(
            ErrorKind::Custom(42),
            optional_expression_expr
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

named!(function_name<&[u8], String, ParseError>,
    map_res!(
        take_while1!(
            is_alphanumeric
        ),
        |bytes: &[u8]| -> Result<String, ParseError> {
            match str::from_utf8(bytes) {
                Ok(string) => Ok(String::from(string)),
                Err(utf8error) => Err(ParseError::StrUnicodeError(utf8error)),
            }
        }
    )
);

fn function_arg_parser(input: &[u8]) -> IResult<&[u8], building::Expression, ParseError> {
    // 2 closing tokens for an argument parser:
    // the argument separator ","
    // and the function closer ")"
    limited_expression_parser(input, &[b',', b')'])
}

named!(function_args<&[u8], Vec<building::Expression>, ParseError>,
    separated_list!(
        tag!(","),
        do_parse!(
            add_return_error!(ErrorKind::Custom(ParseError::NomError),
                take_while!( is_space )
             ) >>
            result: function_arg_parser >>
            (result)
        )
    )
);

fn make_function_item(func_call: (String, Vec<building::Expression>)) -> building::Item {
    let (name, arguments) = func_call;
    let func_call = building::FunctionCall {
        name,
        arguments,
    };
    building::Item::Function(func_call)
}

named!(function_item<&[u8], building::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        map!(
            do_parse!(
                add_return_error!(ErrorKind::Custom(ParseError::NomError), tag!("$")) >>
                // function name
                func_name: function_name >>
                add_return_error!(ErrorKind::Custom(ParseError::NomError), tag!("(")) >>
                // arguments
                args: function_args >>
                add_return_error!(ErrorKind::Custom(ParseError::NomError), tag!(")")) >>
                (func_name, args)),
            make_function_item
        )
    )
);

named!(parse_item<&[u8], building::Item, ParseError>,
    alt!(
        escaped_text |
        item_tag |
        function_item |
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

fn limited_expression_parser<'a>(mut input: &'a [u8], finishing_characters: &[u8]) -> IResult<&'a [u8], building::Expression, ParseError> {
    let mut items: Vec<building::Item> = Vec::new();
    let mut current_text: Vec<u8> = Vec::new();
    'expression_loop: while input.len() > 0 {
        // special characters
        if let Some(_) = finishing_characters.iter().position(|&x| x == input[0]) {
            break 'expression_loop;
        }
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
    flush_text!(&mut current_text, &mut items);
    let expression = building::Expression {
        items,
    };
    IResult::Done(input, expression)
}

fn parse_expression(input: &[u8]) -> IResult<&[u8], building::Expression, ParseError> {
    limited_expression_parser(input, &[])
}

