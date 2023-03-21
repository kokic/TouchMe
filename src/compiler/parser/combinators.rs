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



/// Returns `space_asterisk <&> parser <&> space_asterisk` 
pub fn soft<X>(
    parser: impl Fn(&str) -> Result<(&str, X), parsec::ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), parsec::ParseError> {
    parsec::between(space_asterisk, space_asterisk, parser)
}

/// Returns `space_plus <&> parser <&> space_plus` 
pub fn leak<X>(
    parser: impl Fn(&str) -> Result<(&str, X), parsec::ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), parsec::ParseError> {
    parsec::between(space_plus, space_plus, parser)
}

