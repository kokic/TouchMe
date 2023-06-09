#[derive(Debug, PartialEq)]
pub struct ParseError<'a> {
    pub location: &'a str,
    pub expected: String,
}

impl ParseError<'_> {
    pub fn new<'a>(location: &'a str, expected: &'a str) -> ParseError<'a> {
        ParseError {
            location: location,
            expected: expected.to_string(),
        }
    }
}

pub fn map<F, X, Y>(
    parser: impl Fn(&str) -> Result<(&str, X), ParseError>,
    morph: F,
) -> impl Fn(&str) -> Result<(&str, Y), ParseError>
where
    F: Fn(X) -> Y,
{
    move |input| parser(input).map(|(r, x)| (r, morph(x)))
}

pub fn map_char_to_string(
    parser: impl Fn(&str) -> Result<(&str, char), ParseError>,
) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    map(parser, |x| x.to_string())
}

pub fn follow<A, B>(
    prev: impl Fn(&str) -> Result<(&str, A), ParseError>,
    succ: impl Fn(&str) -> Result<(&str, B), ParseError>,
) -> impl Fn(&str) -> Result<(&str, (A, B)), ParseError> {
    move |input| {
        let (residue, a) = prev(input)?;
        let (residue, b) = succ(residue)?;
        Ok((residue, (a, b)))
    }
}

pub fn append(
    prev: impl Fn(&str) -> Result<(&str, String), ParseError>,
    succ: impl Fn(&str) -> Result<(&str, String), ParseError>,
) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    map(follow(prev, succ), |(s, t)| s + &t)
}

pub fn twice(
    parser: impl Fn(&str) -> Result<(&str, String), ParseError>,
) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    let borrow = parser;
    move |input| append(&borrow, &borrow)(input)
}

pub fn either<A>(
    prev: impl Fn(&str) -> Result<(&str, A), ParseError>,
    succ: impl Fn(&str) -> Result<(&str, A), ParseError>,
) -> impl Fn(&str) -> Result<(&str, A), ParseError> {
    move |input| match prev(input) {
        Ok(x) => Ok(x),
        Err(_) => succ(input),
    }
}

pub fn either3<A>(
    a: impl Fn(&str) -> Result<(&str, A), ParseError>,
    b: impl Fn(&str) -> Result<(&str, A), ParseError>,
    c: impl Fn(&str) -> Result<(&str, A), ParseError>,
) -> impl Fn(&str) -> Result<(&str, A), ParseError> {
    either(either(a, b), c)
}

pub fn either4<A>(
    a: impl Fn(&str) -> Result<(&str, A), ParseError>,
    b: impl Fn(&str) -> Result<(&str, A), ParseError>,
    c: impl Fn(&str) -> Result<(&str, A), ParseError>,
    d: impl Fn(&str) -> Result<(&str, A), ParseError>,
) -> impl Fn(&str) -> Result<(&str, A), ParseError> {
    either(either3(a, b, c), d)
}

pub fn skip<A, B>(
    prev: impl Fn(&str) -> Result<(&str, A), ParseError>,
    succ: impl Fn(&str) -> Result<(&str, B), ParseError>,
) -> impl Fn(&str) -> Result<(&str, A), ParseError> {
    map(follow(prev, succ), |x| x.0)
}

pub fn drop<A, B>(
    prev: impl Fn(&str) -> Result<(&str, A), ParseError>,
    succ: impl Fn(&str) -> Result<(&str, B), ParseError>,
) -> impl Fn(&str) -> Result<(&str, B), ParseError> {
    map(follow(prev, succ), |x| x.1)
}

pub fn many<T>(
    parser: impl Fn(&str) -> Result<(&str, T), ParseError>,
) -> impl Fn(&str) -> Result<(&str, Vec<T>), ParseError> {
    move |input| {
        let mut result = Vec::new();
        let mut remaining_input = input;

        if input.len() >= 1 {
            loop {
                if let Ok((next_input, parse_result)) = parser(remaining_input) {
                    result.push(parse_result);
                    remaining_input = next_input;
                } else {
                    break;
                }
            }
        }

        Ok((remaining_input, result))
    }
}

pub fn some<T>(
    parser: impl Fn(&str) -> Result<(&str, T), ParseError>,
) -> impl Fn(&str) -> Result<(&str, Vec<T>), ParseError> {
    move |input| match many(|x| parser(x))(input) {
        Ok(x) if x.1.len() >= 1 => Ok(x),
        _ => Err(ParseError::new(input, "some length should ge 1")),
    }
}

// pub fn asterisk(
//   parser: impl Fn(&str) -> Result<(&str, String), ParseError>,
// ) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
//   move |input| {
//       let mut result = String::new();
//       let mut remaining_input = input;
//       loop {
//           if let Ok((next_input, parse_result)) = parser(remaining_input) {
//               result += &parse_result;
//               remaining_input = next_input;
//           } else {
//               break;
//           }
//       }
//       Ok((remaining_input, result))
//   }
// }

pub fn asterisk(
    parser: impl Fn(&str) -> Result<(&str, String), ParseError>,
) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    map(many(parser), |x| x.concat())
}

pub fn plus(
    parser: impl Fn(&str) -> Result<(&str, String), ParseError>,
) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    map(some(parser), |x| x.concat())
}

// pub fn str<'a>(
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

pub fn string<'a>(
    expected: &'static str,
) -> impl Fn(&'a str) -> Result<(&'a str, String), ParseError<'a>> {
    move |input| match input.starts_with(expected) {
        true => {
            let len = expected.len();
            Ok((&input[len..], input[..len].to_owned()))
        }
        false => Err(ParseError {
            location: input,
            expected: expected.to_string(),
        }),
    }
}

pub fn token_direct<F>(predicate: F) -> impl Fn(&str) -> Result<(&str, char), ParseError>
where
    F: Fn(char) -> bool,
{
    move |input| {
        let mut chars = input.chars();
        match chars.next() {
            Some(x) if predicate(x) => Ok((chars.as_str(), x)),
            _ => Err(ParseError::new(input, "#token-predicate")),
        }
    }
}

pub fn token<F>(predicate: F) -> impl Fn(&str) -> Result<(&str, String), ParseError>
where
    F: Fn(char) -> bool,
{
    map_char_to_string(token_direct(predicate))
}

pub fn tokens<F>(len: usize, predicate: F) -> impl Fn(&str) -> Result<(&str, String), ParseError>
where
    F: Fn(&str) -> bool,
{
    move |input| match len <= input.len() {
        true => {
            let substr = &input[..len];
            match predicate(substr) {
                true => Ok((&input[len..], substr.to_string())),
                false => Err(ParseError::new(input, "#tokens-predicate")),
            }
        }
        false => Err(ParseError::new(input, "#tokens-len ≤ input.len")),
    }
}

// pub fn character<'a>(
//     expected: char,
// ) -> impl Fn(&'a str) -> Result<(&'a str, String), ParseError<'a>> {
//     move |input| match input.starts_with(expected) {
//         true => Ok((&input[1..], expected.to_string())),
//         false => Err(ParseError {
//             location: input,
//             expected: expected.to_string(),
//         }),
//     }
// }

// pub fn character<'a>(
//     expected: char,
// ) -> impl Fn(&'a str) -> Result<(&'a str, String), ParseError<'a>> {
//     move |input| match token(|x| x == expected)(input) {
//         Ok(x) => Ok(x),
//         _ => Err(ParseError {
//             location: input,
//             expected: format!("start with '{}'", expected),
//         }),
//     }
// }

pub fn character_direct(expected: char) -> impl Fn(&str) -> Result<(&str, char), ParseError> {
    move |input| match token_direct(|x| x == expected)(input) {
        Ok(x) => Ok(x),
        _ => Err(ParseError {
            location: input,
            expected: format!("start with '{}'", expected),
        }),
    }
}

pub fn character(expected: char) -> impl Fn(&str) -> Result<(&str, String), ParseError> {
    map_char_to_string(character_direct(expected))
}

pub fn between<A, B, X>(
    before: impl Fn(&str) -> Result<(&str, A), ParseError>,
    after: impl Fn(&str) -> Result<(&str, B), ParseError>,
    parser: impl Fn(&str) -> Result<(&str, X), ParseError>,
) -> impl Fn(&str) -> Result<(&str, X), ParseError> {
    skip(drop(before, parser), after)
}
