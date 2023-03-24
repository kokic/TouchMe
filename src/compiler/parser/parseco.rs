#[derive(Clone, Debug)]
pub struct State<'a> {
    pub(crate) source: std::str::Chars<'a>,
    pub(crate) locator: Locator,
    // pub(crate) size: usize,
    pub(crate) index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Locator {
    pub column: usize,
    pub row: usize,
}

impl Locator {
    pub fn new(column: usize, row: usize) -> Self {
        Locator { column, row }
    }

    pub fn default(&self) -> Self {
        Locator {
            column: self.column + 1,
            ..*self
        }
    }
}

// impl<'a> State<'a> {
//     pub fn new(src: &'a str) -> Self {
//         State {
//             source: src.chars(),
//             locator: Locator { column: 0, row: 0 },
//             // size: src.len(),
//             index: 0,
//         }
//     }
// }

pub fn state<'a>(input: &'a str) -> State<'a> {
    State {
        source: input.chars(),
        locator: Locator { column: 0, row: 0 },
        // size: src.len(),
        index: 0,
    }
}

impl<'a> Iterator for State<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.source.next()?;
        self.locator = match x {
            '\n' => Locator::new(0, self.locator.row + 1),
            '\t' => Locator {
                column: self.locator.column + 8 - (self.locator.column - 1) % 8,
                ..self.locator
            },
            _ => self.locator.default(),
        };
        self.index += 1;
        Some(x)
    }
}

#[derive(PartialEq)]
pub struct ParserError {
    pub message: String,
}

impl std::fmt::Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("ParserError")
        //     .field("message", &self.message)
        //     .finish()
        f.write_str(&format!("\"{}\"", self.message))
    }
}

impl ParserError {
    pub fn new<'a>(message: &'a str) -> ParserError {
        ParserError {
            message: message.to_string(),
        }
    }
}

pub fn err<'a, X>(message: &'a str) -> Result<X, ParserError> {
    Err(ParserError::new(message))
}

pub fn err_at<'a, X>(message: &'a str, locator: Locator) -> Result<X, ParserError> {
    err(format!("{}: error at {:?}.", message, locator).as_str())
}

pub trait Parser<S> {
    type Value;

    fn parse(&self, state: &mut S) -> Result<Self::Value, ParserError>;
}

#[derive(Clone, Copy, Debug)]
pub struct Satisfied<F> {
    satisfy: F,
}

impl<F> Satisfied<F> {
    pub fn new(satisfy: F) -> Self {
        Self { satisfy }
    }
}

impl<'a, F> Parser<State<'a>> for Satisfied<F>
where
    F: Fn(&char) -> bool,
{
    type Value = char;

    fn parse(&self, state: &mut State<'a>) -> Result<Self::Value, ParserError> {
        match state.source.next() {
            Some(x) if (self.satisfy)(&x) => Ok(x),
            Some(_) => err_at("#Satisfied", state.locator),
            None => err("next failed"),
        }
    }
}

pub fn piece<F>(predicate: F) -> Satisfied<F>
where
    F: Fn(&char) -> bool,
{
    Satisfied::new(predicate)
}

pub fn character(expected: char) -> Satisfied<impl Fn(&char) -> bool> {
    piece(move |x| x == &expected)
}


