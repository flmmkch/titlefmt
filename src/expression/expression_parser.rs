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

fn make_item(string: &str) -> Result<super::Item, ParseError> {
    Ok(super::Item::Tag(string.to_owned()))
}

named!(item_tag<&[u8], super::Item, ParseError>,
    add_return_error!(
        ErrorKind::Custom(ParseError::NomError),
        map_res!(
        delimited!(
            char!('%'),
            map_res!(alphanumeric, str::from_utf8),
            char!('%')),
        make_item
        )
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
        if let Err(err) = flush_text(&mut $x, &mut $y) {
            return IResult::Error(ErrorKind::Custom(err));
        };
    }
}

pub fn parse_expression(mut input: &[u8]) -> IResult<&[u8], super::Expression, ParseError> {
    let mut items: Vec<super::Item> = Vec::new();
    let mut current_text: Vec<u8> = Vec::new();
    while input.len() > 0 {
        let parse_result = item_tag(input);
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
/*
#[test]
fn parse_simple_text() {
  let simple_text_string = &b"simple text string"[..];

  let res = parse(simple_text_string);
  println!("{:?}", res);

  let test = super::Expression {
      items: vec![super::Item::Text("simple text string"),]
  };
  assert_eq!(res, test);
}*/
