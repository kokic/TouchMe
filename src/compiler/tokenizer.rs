

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




pub fn is_identifier_body(x: char) -> bool {
    x.is_alphabetic() || x.is_ascii_digit() || x == '_' || x == '-'
}








#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    pub location: &'a str, 
    pub expected: &'static str, 
    
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




// pub fn string<'a>(
//     expected: &'static str,
// ) -> impl Fn(&'a str) -> Result<(&'a str, &'a str), ParseError<'a>> {
//     move |input| match input.starts_with(expected) {
//         true => {
//             let len = expected.len();
//             Ok((&input[len..], &input[..len]))
//         }
//         false => Err(ParseError{ location: input, expected: expected }),
//     }
// }


pub fn string_owned<'a>(
    expected: &'static str,
) -> impl Fn(&'a str) -> Result<(&'a str, String), ParseError<'a>> {
    move |input| match input.starts_with(expected) {
        true => {
            let len = expected.len();
            Ok((&input[len..], input[..len].to_owned()))
        }
        false => Err(ParseError{ location: input, expected: expected }),
    }
}
