use super::parsec;

pub fn space(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::character(' ')(input)
}

pub fn space_asterisk(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::asterisk(space)(input)
}

pub fn space_plus(input: &str) -> Result<(&str, String), parsec::ParseError> {
    parsec::plus(space)(input)
}

pub fn soft<X>(
    parser: impl Fn(&str) -> Result<(&str, X), parsec::ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), parsec::ParseError> {
    parsec::between(space_asterisk, space_asterisk, parser)
}
