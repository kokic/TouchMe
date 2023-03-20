enum TouchMeTokenCategory {
    EOF,
    Error,
    Comment,
    String,
    Number,
    Identifier,
    Macro,
}

struct TouchMeToken {
    category: TouchMeTokenCategory,
    lineno: u32,
    columno: u32,
    position: u32,
    length: u32,
    filename: String,
    value: String,
}

pub struct TouchMeTokenBundle {
    buffer: String,
    offset: u32,
    position: u32,
    length: u32,
    lineno: u32,
    columno: u32,
    filename: String,

    cache: TouchMeToken,
}

pub fn is_quotes(char: char) -> bool {
    (char == '"') || (char == '\'')
}

pub fn is_builtin_operator(char: char) -> bool {
    (char == '+')
        || (char == '-')
        || (char == '*')
        || (char == '/')
        || (char == '<')
        || (char == '>')
        || (char == '!')
        || (char == '=')
        || (char == '|')
        || (char == '&')
        || (char == '^')
        || (char == '%')
        || (char == '~')
        || (char == '.')
        || (char == ':')
        || (char == '?')
        || (char == ',')
        || (char == '[')
        || (char == ']')
        || (char == '(')
        || (char == ')')
}
