use super::{
    ast::{BinaryExpr, Expr, FunctionExpr},
    parser::{
        combinators::{self},
        parsec::{self, character, tokens},
    },
    tokenizer::{identifier},
};


pub fn primary_expr(input: &str) -> Result<(&str, Expr), parsec::ParseError> {
    parsec::map(identifier, |x| Expr::Identifier(x))(input)
}

pub fn add_expr(input: &str) -> Result<(&str, Expr), parsec::ParseError> {
    let add_infix = combinators::leak(character('+'));
    let parser = parsec::follow(parsec::follow(identifier, add_infix), identifier);
    let morph = |x: ((String, String), String)| {
        Expr::Add(Box::new(BinaryExpr {
            operator: x.0 .1,
            lhs: Expr::Identifier(x.0 .0),
            rhs: Expr::Identifier(x.1),
        }))
    };
    parsec::map(parser, morph)(input)
}

pub fn expr(input: &str) -> Result<(&str, Expr), parsec::ParseError> {
    parsec::either(add_expr, primary_expr)(input)
}

/// match `soft ->` or `soft =>`
pub fn arrow(input: &str) -> Result<(&str, String), parsec::ParseError> {
    combinators::soft(tokens(2, |x| x == "->" || x == "=>"))(input)
}

/// parameters must contain at least one parameter
///
/// e.g. `x` or `x y`
pub fn parameters(input: &str) -> Result<(&str, Vec<Expr>), parsec::ParseError> {
    parsec::map(parsec::some(combinators::soft(identifier)), |xs| {
        xs.iter().map(|x| Expr::Identifier(x.to_string())).collect()
    })(input)
}

/// accelerator function
/// - is expression
/// - is anonymous function
/// - is arrow function (ECMAScript Language Specification)
///
/// e.g. `x y -> x + y` or `x y => x + y`
pub fn function(input: &str) -> Result<(&str, Expr), parsec::ParseError> {
    let parser = parsec::follow(parsec::skip(parameters, arrow), expr);
    let morph = |x: (Vec<Expr>, Expr)| {
        Expr::Function(Box::new(FunctionExpr {
            params: x.0,
            body: x.1,
        }))
    };
    parsec::map(parser, morph)(input)
}
