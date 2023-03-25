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

pub trait Parser<A> {
    type Value;

    fn parse(&self, state: &mut A) -> Result<Self::Value, ParserError>;

    fn map<B, F>(self, morph: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Value) -> B,
    {
        Map::new(self, morph)
    }

    fn twice(self) -> Twice<Self>
    where
        Self: Parser<A, Value = String> + Sized,
    {
        Twice { parser: self }
    }

    fn or<B>(self, succ: B) -> Or<Self, B>
    where
        Self: Sized,
        // B: Parser<A, Value = Self::Value>,
    {
        Or::new(self, succ)
    }
}

// #[derive(Clone, Copy, Debug)]
// pub struct Twice<A> {
//     parser: A,
// }


// #[derive(Clone, Copy)]
pub struct Map<P, F> {
    parser: P,
    morph: F,
}

impl<P, F> Map<P, F> {
    pub fn new(parser: P, morph: F) -> Self {
        Self { parser, morph }
    }
}

impl<S, B, P: Parser<S>, F> Parser<S> for Map<P, F>
where
    F: Fn(P::Value) -> B,
{
    type Value = B;

    fn parse(&self, stream: &mut S) -> Result<Self::Value, ParserError> {
        self.parser.parse(stream).map(&self.morph)
    }
}






pub struct Twice<A> {
    parser: A,
}

impl<S, A> Parser<S> for Twice<A>
where
    A: Parser<S, Value = String>,
{
    type Value = A::Value;

    fn parse(&self, state: &mut S) -> Result<Self::Value, ParserError> {
        match (self.parser.parse(state), self.parser.parse(state)) {
            (Ok(s), Ok(t)) => Ok(s + &t),
            (Err(_), _) => err("twice but failed at first"),
            (_, Err(_)) => err("twice but failed at second"),
        }
    }
}




#[derive(Clone, Copy, Debug)]
pub struct Or<A, B> {
    prev: A,
    succ: B,
}

impl<A, B> Or<A, B> {
    pub fn new(prev: A, succ: B) -> Self {
        Self { prev, succ }
    }
}

impl<S: Clone, A, B> Parser<S> for Or<A, B>
where
    A: Parser<S>,
    B: Parser<S, Value = A::Value>,
{
    type Value = A::Value;

    fn parse(&self, state: &mut S) -> Result<Self::Value, ParserError> {
        let mut state_copied = state.clone();
        match self.prev.parse(state) {
            Ok(x) => Ok(x),
            Err(_) => self.succ.parse(&mut state_copied),
        }
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


#[derive(Clone, Copy, Debug)]
pub struct Satisfied<F> {
    satisfy: F,
}

impl<F> Satisfied<F> {
    pub fn new(satisfy: F) -> Self {
        Self { satisfy }
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
