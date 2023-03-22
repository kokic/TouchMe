pub struct ParseError<'a> {
    pub location: &'a str,
    pub expected: String,
}

pub struct Parser<'a, A> {
    parse: Box<dyn Fn(&'a str) -> Result<(&'a str, A), ParseError<'a>> + 'a>,
}

impl<'a, A> Parser<'a, A> {
    pub fn new<F: 'a + Fn(&'a str) -> Result<(&'a str, A), ParseError<'a>>>(f: F) -> Self {
        Parser::<A> { parse: Box::new(f) }
    }

    pub fn parse(&self, input: &'a str) -> Result<(&'a str, A), ParseError<'a>> {
        (self.parse)(input)
    }

    // pub fn either(&self, other: &Parser<'a, A>) -> Parser<'a, A> {
    //     Parser::new(move |input| {
    //         match (self.parse(input), other.parse(input)) {
    //             (Ok(result), _) => Ok(result),
    //             (Err(_), Ok(result)) => Ok(result),
    //             (Err(err), Err(_)) => Err(err),
    //         }
    //     })
    // }
}
