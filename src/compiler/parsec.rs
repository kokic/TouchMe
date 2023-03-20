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

pub fn map<F, X, Y>(
    parser: impl Fn(&str) -> Result<(&str, X), ParseError>,
    morph: F,
) -> impl Fn(&str) -> Result<(&str, Y), ParseError>
where
    F: Fn(X) -> Y,
{
    move |input| parser(input).map(|(r, x)| (r, morph(x)))
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

pub fn some<T>(
    parser: impl Fn(&str) -> Result<(&str, T), ParseError>,
) -> impl Fn(&str) -> Result<(&str, Vec<T>), ParseError> {
    move |input| {
        match many(|x| parser(x))(input) {
            Ok(x) if x.1.len() >= 1 => Ok(x),
            _ => Err(ParseError::new(input, "some length should ge 1")),
        }
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

pub fn token<F>(predicate: F) -> impl Fn(&str) -> Result<(&str, String), ParseError>
where
    F: Fn(char) -> bool,
{
    move |input| {
        let mut chars = input.chars();
        match chars.next() {
            Some(x) if predicate(x) => Ok((chars.as_str(), x.to_string())),
            _ => Err(ParseError::new(input, "#token-predicate")),
        }
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

pub fn character<'a>(
    expected: char,
) -> impl Fn(&'a str) -> Result<(&'a str, String), ParseError<'a>> {
    move |input| match token(|x| x == expected)(input) {
        Ok(x) => Ok(x),
        _ => Err(ParseError {
            location: input,
            expected: format!("start with '{}'", expected),
        }),
    }
}
