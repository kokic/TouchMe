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

pub fn is_identifier_head(x: char) -> bool {
    x.is_alphabetic() || x == '_'
}

pub fn is_identifier_body(x: char) -> bool {
    is_identifier_head(x) || x.is_ascii_digit() || x == '-'
}



