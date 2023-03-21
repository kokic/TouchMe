use crate::compiler::tokenizer;

pub mod compiler;

/*
use std::io::{stdin, stdout, Write};


#[derive(Clone)]
enum Val {
    Number(f64),
    Boolean(bool),
    Symbol(String),
    List(Vec<Val>),
    Function(fn(&[Val]) -> Val),
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "#unsupported");

        write!(
            f,
            "{}",
            match self {
                Val::Number(n) => n.to_string(),
                Val::Boolean(b) => b.to_string(),
                Val::Symbol(sym) => sym.to_string(),
                Val::List(_) => "#todo".to_string(),
                Val::Function(_) => "#todo".to_string(),
            }
        )
    }
}

fn parse(tokens: &mut Vec<String>) -> Val {
    if tokens.len() == 0 {
        panic!("Unexpected EOF");
    }

    let tok = tokens.remove(0);

    if tok == "(" {
        let mut list = Vec::new();
        while tokens[0] != ")" {
            list.push(parse(tokens));
        }
        tokens.remove(0);
        return Val::List(list);
    } else if tok == "true" {
        Val::Boolean(true)
    } else if tok == "false" {
        Val::Boolean(false)
    }
    /* else if tok == "#" {
        let bool_token = tokens.remove(0);
        return match bool_token.as_str() {
            "t" => Val::Boolean(true),
            "f" => Val::Boolean(false),
            _ => panic!("Invalid bool value"),
        };
    } */
    else {
        return match tok.parse() {
            Ok(num) => Val::Number(num),
            Err(_) => Val::Symbol(tok),
        };
    }
}

fn eval(expr: &Val, env: &mut HashMap<String, Val>) -> Val {
    match expr {
        Val::Number(n) => Val::Number(*n),
        Val::Boolean(b) => Val::Boolean(*b),
        Val::Symbol(sym) => match env.get(sym) {
            Some(val) => val.clone(),
            None => panic!("Undefined symbol {}", sym),
        },
        Val::List(list) => {
            let first = &list[0];
            let args = &list[1..];

            match first {
                Val::Symbol(sym) => match sym.as_str() {
                    "def" => {
                        let name = match &args[0] {
                            Val::Symbol(s) => s.clone(),
                            _ => panic!("def requires a symbol as the first argument"),
                        };
                        let value = eval(&args[1], env);
                        env.insert(name, value);
                        Val::Number(0.0)
                    }
                    _ => match eval(&first, env) {
                        Val::Function(f) => {
                            f(&args.iter().map(|arg| eval(arg, env)).collect::<Vec<Val>>())
                        }
                        _ => panic!("Invalid function call"),
                    },
                },
                _ => panic!("Invalid function call"),
            }
        }
        _ => panic!("Invalid expression"),
    }
}

fn number_reduce<F>(args: &[Val], f: F) -> Val
where
    F: FnMut(f64, f64) -> f64,
{
    Val::Number(
        args.iter()
            .filter_map(|arg| match arg {
                Val::Number(n) => Some(*n),
                _ => None,
            })
            .reduce(f)
            .unwrap(),
    )
}

fn add(args: &[Val]) -> Val {
    number_reduce(args, |x, y| x + y)
}

fn sub(args: &[Val]) -> Val {
    number_reduce(args, |x, y| x - y)
}

fn mul(args: &[Val]) -> Val {
    number_reduce(args, |x, y| x * y)
}

fn div(args: &[Val]) -> Val {
    number_reduce(args, |x, y| x / y)
}

*/

fn parse_a(input: &str) -> Result<(&str, String), compiler::parsec::ParseError> {
    compiler::parsec::character('a')(input)
}

fn parse_b(input: &str) -> Result<(&str, String), compiler::parsec::ParseError> {
    compiler::parsec::character('b')(input)
}

fn main() {
    // let parser = compiler::tokenizer::Parser { input: "abc" };
    // let letter_a = parser.string("a");

    // let letter_a = compiler::tokenizer::token("a");

    let _a_many = compiler::parsec::many(parse_a);
    let _a_plus = compiler::parsec::plus(parse_a);

    let _a_a = compiler::parsec::follow(parse_a, parse_a);

    println!("{:?}", _a_plus("aabc"));
    println!("{:?}", tokenizer::identifier("good-name"));

    let _tokens = compiler::parsec::tokens(|s| s.starts_with("if "), 3);


    let _a_or_b = compiler::parsec::either(parse_a, parse_b);
    println!("{:?}", _a_or_b("bax"));

    // let mut env = HashMap::new();
    // env.insert("+".to_string(), Val::Function(add));
    // env.insert("add".to_string(), Val::Function(add));
    // env.insert("sum".to_string(), Val::Function(add));

    // env.insert("-".to_string(), Val::Function(sub));
    // env.insert("sub".to_string(), Val::Function(sub));

    // env.insert("*".to_string(), Val::Function(mul));
    // env.insert("mul".to_string(), Val::Function(mul));
    // env.insert("prod".to_string(), Val::Function(mul));

    // env.insert("/".to_string(), Val::Function(div));
    // env.insert("div".to_string(), Val::Function(div));

    // loop {
    //     let mut input = String::new();
    //     print!(">>> ");
    //     stdout().flush().expect("flush failed");
    //     stdin().read_line(&mut input).expect("read failed");
    //     let mut tokens: Vec<_> = input.split_whitespace().map(|s| s.to_string()).collect();

    //     let expr = parse(&mut tokens);
    //     let result = eval(&expr, &mut env);
    //     println!("{}", result);
    // }
}
