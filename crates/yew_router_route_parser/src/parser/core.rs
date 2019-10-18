//! Core functions for working with the route parser.
use crate::parser::CaptureOrExact;
use crate::parser::RouteParserToken;
use crate::parser::{RefCaptureVariant};
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take};
use nom::character::complete::char;
use nom::character::complete::digit1;
use nom::character::is_digit;
use nom::combinator::{map, opt, peek};
use nom::error::ParseError;
use nom::error::{context, ErrorKind, VerboseError};
use nom::multi::separated_list;
use nom::sequence::{delimited, pair, preceded, separated_pair};
use nom::IResult;

/// Captures a string up to the point where a character not possible to be present in Rust's identifier is encountered.
/// It prevents the first character from being a digit.
pub fn valid_ident_characters(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    const INVALID_CHARACTERS: &str = " -*/+#?&^@%$\'\"`%~;,.|\\{}[]()<>=\t\n";
    context("valid ident", |i: &str| {
        let (i, next) = peek(take(1usize))(i)?; // Look at the first character
        if is_digit(next.bytes().next().unwrap()) {
            Err(nom::Err::Error(VerboseError::from_error_kind(
                i,
                ErrorKind::Digit,
            )))
        } else {
            is_not(INVALID_CHARACTERS)(i)
        }
    })(i)
}

/// A more permissive set of characters than those specified in `valid_ident_characters that the route string will need to match exactly.
pub fn valid_exact_match_characters(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    const INVALID_CHARACTERS: &str = " /?&#=\t\n()|[]{}";
    context("valid exact match", |i: &str| is_not(INVALID_CHARACTERS)(i))(i)
}

/// Captures groups of characters that will need to be matched exactly later.
pub fn match_exact(i: &str) -> IResult<&str, RouteParserToken, VerboseError<&str>> {
    context(
        "match",
        map(valid_exact_match_characters, |ident| {
            RouteParserToken::Exact(ident)
        }),
    )(i)
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn rejects_invalid_ident() {
        valid_ident_characters("+-lorem").expect_err("Should reject at +");
    }

    #[test]
    fn accepts_valid_ident() {
        valid_ident_characters("Lorem").expect("Should accept");
    }

}
