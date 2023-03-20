use std::collections::HashMap;


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

pub fn is_quotes(x: char) -> bool {
    (x == '"') || (x == '\'')
}

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

pub fn is_identifier(x: char) -> bool {
    x.is_alphabetic() || x.is_ascii_digit() || x == '_' || x == '-'
}










pub fn token<'a>(
    expected: &'static str,
) -> impl Fn(&'a str) -> Result<(&'a str, &'a str), ParseError<'a>> {
    move |input| match input.starts_with(expected) {
        true => {
            let len = expected.len();
            Ok((&input[len..], &input[..len]))
        }
        false => Err(ParseError{ location: input, expected: expected }),
    }
}


pub fn many<T>(
    parser: impl Fn(&str) -> Result<(&str, T), ParseError>,
) -> impl Fn(&str) -> Result<(&str, Vec<T>), ParseError> {
    move |input| {
        let mut result = Vec::new();
        let mut remaining_input = input;

        loop {
            if let Ok((next_input, parse_result)) = parser(remaining_input) {
                result.push(parse_result);
                remaining_input = next_input;
            } else {
                break;
            }
        }

        Ok((remaining_input, result))
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    pub location: &'a str, 
    pub expected: &'static str, 
    
}




 pub enum Element {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<Element>),
    Object(HashMap<String, Element>),
}


pub struct Parser<'a> {
    pub input: &'a str,
}

impl<'a> Parser<'a> {

    pub fn string(&self, pattern: &str) -> Option<(&str, ())> {
        let mut iter = pattern.chars();

        for p in self.input.chars() {
            match iter.next() {
                Some(c) if c == p => {}
                _ => {
                    return None;
                }
            }

            if iter.as_str().is_empty() {
                let rest = &self.input[pattern.len()..];
                return Some((rest, ()));
            }
        }
        None
    }

    // fn number(&self) -> Option<(&str, Element)> {
    //     let mut chars = self.input.chars().peekable();
    //     let mut num_str = String::new();

    //     while let Some(ch) = chars.peek() {
    //         if (*ch >= '0' && *ch <= '9') || *ch == '-' || *ch == '+' || *ch == '.' || *ch == 'e' {
    //             num_str.push(*ch);
    //         } else {
    //             break;
    //         }
    //         chars.next();
    //     }

    //     if num_str.is_empty() {
    //         None
    //     } else {
    //         let rest = chars.as_str();
    //         Some((rest, Element::Number(num_str.parse().unwrap())))
    //     }
    // }

    pub fn boolean(&self) -> Option<(&str, Element)> {
        let pattern_true = "true";
        let pattern_false = "false";
        match self.string(pattern_true) {
            Some((rest, _)) => Some((rest, Element::Boolean(true))),
            None => match self.string(pattern_false) {
                Some((rest, _)) => Some((rest, Element::Boolean(false))),
                None => None,
            },
        }
    }

    pub fn null(&self) -> Option<(&str, Element)> {
        match self.string("null") {
            Some((rest, _)) => Some((rest, Element::Null)),
            None => None,
        }
    }

    pub fn comma(&self) -> Option<(&str, ())> {
        self.string(",").map(|(rest, _)| (rest, ()))
    }





    // fn array(&self) -> Option<(&str, Element)> {
    //     let mut elements = Vec::new();
    //     let mut rest = self.input;
    //     loop {
    //         match self.element()(rest) {
    //             Some((r, elem)) => {
    //                 rest = r.trim_start();
    //                 elements.push(elem);
    //             }
    //             None => break,
    //         }

    //         match self.comma()(rest) {
    //             Some((r, _)) => rest = r,
    //             None => break,
    //         }
    //     }

    //     match self.string("]") {
    //         Some((rest, _)) => Some((rest, Element::Array(elements))),
    //         None => None,
    //     }
    // }

    // fn object_key(&self) -> Option<(&str, String)> {
    //     let mut chars = self.input.chars().peekable();
    //     let mut key = String::new();

    //     while let Some(ch) = chars.next() {
    //         if ch == '"' {
    //             while let Some(next_ch) = chars.peek() {
    //                 if *next_ch == '"' {
    //                     chars.next();
    //                     return Some((&chars.as_str()[1..], key));
    //                 }
    //                 key.push(chars.next().unwrap());
    //             }
    //         } else if !ch.is_whitespace() {
    //             return None;
    //         }
    //     }

    //     None
    // }

    // fn object(&self) -> Option<(&str, Element)> {
    //     let mut object = HashMap::new();
    //     let mut rest = self.input;
    //     loop {
    //         match self.object_key()(rest) {
    //             Some((r, key)) => {
    //                 rest = r.trim_start();
    //                 if let Some((r, _)) = self.string(":")(rest) {
    //                     rest = r.trim_start();
    //                     if let Some((r, value)) = self.element()(rest) {
    //                         rest = r.trim_start();
    //                         object.insert(key, value);
    //                     }
    //                 }
    //             }
    //             None => break,
    //         }

    //         match self.comma()(rest) {
    //             Some((r, _)) => rest = r,
    //             None => break,
    //         }
    //     }

    //     match self.string("}") {
    //         Some((rest, _)) => Some((rest, Element::Object(object))),
    //         None => None,
    //     }
    // }
    
    // fn element(&self) -> impl Fn(&str) -> Option<(&str, Element)> {
    //     move |input: &str| {
    //         Parser { input }.string("\\\\\"").and_then(|(rest, _)| {
    //             let mut chars = rest.chars();
    //             let mut s = String::new();

    //             while let Some(ch) = chars.next() {
    //                 if ch == '\\' {
    //                     match chars.next() {
    //                         Some('"') => s.push('"'),
    //                         Some('\\') => s.push('\\'),
    //                         Some('/') => s.push('/'),
    //                         Some('b') => s.push('\u{0008}'),
    //                         Some('f') => s.push('\u{000C}'),
    //                         Some('n') => s.push('\n'),
    //                         Some('r') => s.push('\r'),
    //                         Some('t') => s.push('\t'),
    //                         Some('u') => {
    //                             let uc: String = chars.by_ref().take(4).collect();
    //                             match u32::from_str_radix(&uc, 16) {
    //                                 Ok(cp) => {
    //                                     match char::from_u32(cp) {
    //                                         Some(c) => s.push(c),
    //                                         None => {
    //                                             return None;
    //                                         }
    //                                     };
    //                                 }
    //                                 Err(_) => {
    //                                     return None;
    //                                 }
    //                             };
    //                         }
    //                         _ => {
    //                             return None;
    //                         }
    //                     }
    //                 } else if ch == '"' {
    //                     return Some((&rest[chars.as_str().len()..], Element::String(s)));
    //                 } else {
    //                     s.push(ch)
    //                 }
    //             }

    //             None
    //         }).or_else(|| self.number())
    //           .or_else(|| self.boolean())
    //           .or_else(|| self.null())
    //           .or_else(|| self.array())
    //           .or_else(|| self.object())
    //     }
    // }
    
}
