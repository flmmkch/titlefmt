use nom::{IResult, alphanumeric};
use std::str;
use std::string;
use nom::ErrorKind;

#[derive(Debug)]
pub enum ParseError {
    NomError,
    Incomplete,
    UnicodeError(string::FromUtf8Error),
    Unknown,
}

fn make_escaped_text_item(string: &str) -> Result<super::Item, ParseError> {
    Ok(super::Item::Text(string.to_owned()))
}

named!(escaped_text<&[u8], super::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        alt!(
            // special rule: '' => turns to a single ' text
            value!(
                super::Item::Text("'".to_owned()),
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

fn make_tag_item(string: &str) -> Result<super::Item, ParseError> {
    Ok(super::Item::Tag(string.to_owned()))
}

named!(item_tag<&[u8], super::Item, ParseError>,
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

fn make_expression_box(expression: super::Expression) -> Result<Box<super::Expression>, ParseError> {
    Ok(Box::new(expression))
}

named!(optional_expression<&[u8], Box<super::Expression>>,
    map_res!(
        add_return_error!(
            ErrorKind::Custom(42),
            parse_expression
            ),
        make_expression_box
    )
);

fn make_optional_item(expression: Box<super::Expression>) -> Result<super::Item, ParseError> {
    Ok(super::Item::OptionalExpr(expression))
}

named!(item_optional<&[u8], super::Item, ParseError>,
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

named!(parse_item<&[u8], super::Item, ParseError>,
    alt!(
        escaped_text |
        item_tag |
        item_optional
    )
);

fn flush_text(current_text: &mut Vec<u8>, items: &mut Vec<super::Item>) -> Result<(), ParseError> {
    if current_text.len() > 0 {
        let text_result = String::from_utf8(current_text.to_vec());
        match text_result {
            Ok(text) => {
                items.push(super::Item::Text(text));
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

pub fn parse_expression(mut input: &[u8]) -> IResult<&[u8], super::Expression, ParseError> {
    let mut items: Vec<super::Item> = Vec::new();
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
    let expression = super::Expression {
        items,
    };
    IResult::Done(input, expression)
}

pub fn parse(string: &str) -> Result<super::Expression, ParseError> {
    let result = parse_expression(string.as_bytes());
    match result {
        IResult::Done(_, expression) => Ok(expression),
        IResult::Error(ErrorKind::Custom(err)) => Err(err),
        _ => Err(ParseError::Unknown),
    }
}
