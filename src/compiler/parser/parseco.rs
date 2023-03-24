use super::parsec::ParseError;

#[derive(Clone, Debug)]
pub struct State<'a> {
    pub(crate) source: std::str::Chars<'a>,
    pub(crate) locator: Locator,
    pub(crate) size: usize,
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

impl<'a> State<'a> {
    pub fn new(src: &'a str) -> Self {
        State {
            source: src.chars(),
            locator: Locator { column: 0, row: 0 },
            size: src.len(),
            index: 0,
        }
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
            _ => self.locator.default()
        };
        self.index += 1;
        Some(x)
    }
}

pub trait Parser<S> {
    type Value;

    fn parse(&self, state: &mut S) -> Result<(&str, Self::Value), ParseError>;
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

    fn parse(&self, state: &mut State<'a>) -> Result<(&str, Self::Value), ParseError> {
        match state.source.next() {
            Some(x) if (self.satisfy)(&x) => Ok(("", x)),
            _ => Err(ParseError::new("", "#token-predicate")),
        }
    }
}

pub fn satisfy<F>(f: F) -> Satisfied<F>
where
    F: Fn(&char) -> bool,
{
    Satisfied::new(f)
}
