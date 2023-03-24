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

use super::parser::{
    combinators,
    parsec::{self, character, plus, token},
};

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

pub fn builtin_operator(input: &str) -> Result<(&str, String), parsec::ParseError> {
    combinators::leak(token(is_builtin_operator))(input)
}

pub fn is_quotes(x: char) -> bool {
    (x == '"') || (x == '\'')
}

pub fn quote(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::token(is_quotes)(input)
}

pub fn left_corner_bracket(input: &str) -> Result<(&str, String), parsec::ParseError> {
    character('「')(input)
}

pub fn right_corner_bracket(input: &str) -> Result<(&str, String), parsec::ParseError> {
    character('」')(input)
}

pub fn left_and_right_corner_bracket(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::append(left_corner_bracket, right_corner_bracket)(input)
}

pub fn string_of<X>(
    parser: impl Fn(&str) -> Result<(&str, X), parsec::ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), parsec::ParseError> {
    let quote_left = parsec::either(quote, left_corner_bracket);
    let quote_right = parsec::either(quote, right_corner_bracket);
    parsec::between(quote_left, quote_right, parser)
}

/// must be not empty
pub fn valid_string_content(input: &str) -> Result<(&str, String), parsec::ParseError> {
    plus(token(|x| x != '\'' && x != '"' && x != '」'))(input)
}


/// match string of
/// - empty `''` or `""` or `「」`
/// - `valid_string_content`
pub fn string(input: &str) -> Result<(&str, String), parsec::ParseError> {

    // of(quote)
    //     .twice()
    //     .either(of(left_and_right_corner_bracket))
    //     .either(of(string_of(valid_string_content)))
    //     .parse(input)

    todo!()
}

pub fn is_identifier_head(x: char) -> bool {
    x.is_alphabetic() || x == '_' || x == '$'
}

pub fn is_identifier_body(x: char) -> bool {
    is_identifier_head(x) || x.is_ascii_digit() || x == '-' || x == '\''
}

pub fn identifier(input: &str) -> Result<(&str, String), parsec::ParseError> {
    let head = parsec::token(is_identifier_head);
    let body = parsec::asterisk(parsec::token(is_identifier_body));
    parsec::map(parsec::follow(head, body), |(s, t)| s + &t)(input)
}
