// enum TouchMeTokenCategory {
//     EOF,
//     Error,
//     Comment,
//     String,
//     Number,
//     Identifier,
//     Macro,
// }

// struct TouchMeToken {
//     category: TouchMeTokenCategory,
//     lineno: u32,
//     columno: u32,
//     position: u32,
//     length: u32,
//     filename: String,
//     value: String,
// }

// pub struct TouchMeTokenBundle {
//     buffer: String,
//     offset: u32,
//     position: u32,
//     length: u32,
//     lineno: u32,
//     columno: u32,
//     filename: String,

//     cache: TouchMeToken,
// }

use super::parser::parsec::{self, character};

pub fn is_builtin_operator(x: char) -> bool {
    (x == '+')
        || (x == '-')
        || (x == '*')
        || (x == '/')
        || (x == '<')
        || (x == '>')
        || (x == '!')
        || (x == '=')
        || (x == '|')
        || (x == '&')
        || (x == '^')
        || (x == '%')
        || (x == '~')
        || (x == '.')
        || (x == ':')
        || (x == '?')
        || (x == ',')
        || (x == '[')
        || (x == ']')
        || (x == '(')
        || (x == ')')
}

pub fn is_quotes(x: char) -> bool {
    (x == '"') || (x == '\'')
}

pub fn quote(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::token(is_quotes)(input)
}

pub fn string<X>(
    parser: impl Fn(&str) -> Result<(&str, X), parsec::ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), parsec::ParseError> {
    let quote_left = parsec::either(quote, character('「'));
    let quote_right = parsec::either(quote, character('」'));
    parsec::between(quote_left, quote_right, parser)
}

pub fn is_identifier_head(x: char) -> bool {
    x.is_alphabetic() || x == '_'
}

pub fn is_identifier_body(x: char) -> bool {
    is_identifier_head(x) || x.is_ascii_digit() || x == '-'
}

pub fn identifier(input: &str) -> Result<(&str, String), parsec::ParseError> {
    let head = parsec::token(is_identifier_head);
    let body = parsec::asterisk(parsec::token(is_identifier_body));
    parsec::map(parsec::follow(head, body), |(s, t)| s + &t)(input)
}
