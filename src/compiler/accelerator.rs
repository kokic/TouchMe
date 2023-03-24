use super::{
    ast::{BinaryExpr, Expr, FunctionExpr},
    parser::{
        combinators::{self},
        parsec::{self, between, character, token, tokens},
    },
    tokenizer::{valid_string_content, identifier},
};

/// if cannot find value (`identifier`) in this scope then parse it as string
pub fn primary_expr(input: &str) -> Result<(&str, Expr), parsec::ParseError> {

    let paren = 
    // parsec::of(expr)
    // .between(character('('), character(')'));
    
    between(character('('), character(')'), expr);


    
    let identifier = parsec::map(identifier, |x| Expr::Identifier(x));
    let number = parsec::map(token(|x| x.is_digit(10)), |x| {
        Expr::Integer(x.parse::<i64>().unwrap())
    });
    let as_string = parsec::map(valid_string_content, |x| Expr::String(x));

    parsec::either4(paren, identifier, number, as_string)(input)
}

pub fn add_expr(input: &str) -> Result<(&str, Expr), parsec::ParseError> {
    let add_infix = combinators::leak(character('+'));
    let parser = parsec::follow(parsec::follow(primary_expr, add_infix), primary_expr);
    let morph = |x: ((Expr, String), Expr)| {
        Expr::Add(Box::new(BinaryExpr {
            operator: x.0 .1,
            lhs: x.0 .0,
            rhs: x.1,
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



