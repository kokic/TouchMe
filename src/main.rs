use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

#[derive(Clone)]
enum Val {
    Num(i64),
    Bool(bool),
    Symbol(String),
    List(Vec<Val>),
    Func(fn(&[Val]) -> Val),
}

impl std::fmt::Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "#unsupported");

        write!(f, "{}", match self {
            Val::Num(n) => n.to_string(),
            Val::Bool(b) => b.to_string(),
            Val::Symbol(sym) => sym.to_string(),
            Val::List(_) => "#todo".to_string(),
            Val::Func(_) => "#todo".to_string(),
        })
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
    } else if tok == "#" {
        let bool_token = tokens.remove(0);
        return match bool_token.as_str() {
            "t" => Val::Bool(true),
            "f" => Val::Bool(false),
            _ => panic!("Invalid bool value"),
        };
    } else {
        return match tok.parse() {
            Ok(num) => Val::Num(num),
            Err(_) => Val::Symbol(tok),
        };
    }
}

fn eval(expr: &Val, env: &mut HashMap<String, Val>) -> Val {
    match expr {
        Val::Num(n) => Val::Num(*n),
        Val::Bool(b) => Val::Bool(*b),
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
                        Val::Num(0)
                    }
                    _ => match eval(&first, env) {
                        Val::Func(f) => f(&args
                            .iter()
                            .map(|arg| eval(arg, env))
                            .collect::<Vec<Val>>()),
                        _ => panic!("Invalid function call"),
                    },
                },
                _ => panic!("Invalid function call"),
            }
        }
        _ => panic!("Invalid expression"),
    }
}

fn add(args: &[Val]) -> Val {
    Val::Num(
        args.iter()
            .filter_map(|arg| match arg {
                Val::Num(n) => Some(*n),
                _ => None,
            })
            .sum(),
    )
}

fn parse_float(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}



fn main() {
    let mut env = HashMap::new();
    env.insert("+".to_string(), Val::Func(add));
    env.insert("add".to_string(), Val::Func(add));
    env.insert("sum".to_string(), Val::Func(add));

    loop {
        let mut input = String::new();
        print!(">>> ");
        stdout().flush().expect("flush failed");
        stdin().read_line(&mut input).expect("read failed");
        let mut tokens: Vec<_> = input.split_whitespace().map(|s| s.to_string()).collect();

        let expr = parse(&mut tokens);
        let result = eval(&expr, &mut env);
        println!("{}", result);

    }
}
