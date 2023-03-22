use super::{
    parser::{
        combinators::{self, soft},
        parsec::{self, tokens},
    },
    tokenizer::identifier,
};

/// `->` or `=>`
pub fn arrow(input: &str) -> Result<(&str, String), parsec::ParseError> {
    soft(tokens(2, |x| x == "->" || x == "=>"))(input)
}

/// parameters must contain at least one parameter
pub fn parameters(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::plus(combinators::leak(identifier))(input)
}

/// accelerator function
/// - is expression
/// - is anonymous function
/// - is arrow function (ECMAScript Language Specification)
///
/// e.g. `x y -> x + y` or `x y => x + y`
pub fn function(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::skip(parameters, arrow)(input)
}
