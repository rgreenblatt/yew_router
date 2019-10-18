use crate::parser::parse;
use crate::parser::RouteParserToken;
use crate::parser::{CaptureOrExact};
use nom::IResult;
use std::iter::Peekable;
use std::slice::Iter;
//use nom::bytes::complete::take_till1;
use crate::parser::util::alternative;
use crate::parser::YewRouterParseError;
use crate::CaptureVariant;
use nom::combinator::{map, };

/// Tokens used to determine how to match and capture sections from a URL.
#[derive(Debug, PartialEq, Clone)]
pub enum MatcherToken {
    /// Section-related tokens can be condensed into a match.
    Exact(String),
    /// Capture section.
    Capture(CaptureVariant),
}

impl From<CaptureOrExact> for MatcherToken {
    fn from(value: CaptureOrExact) -> Self {
        match value {
            CaptureOrExact::Exact(m) => MatcherToken::Exact(m),
            CaptureOrExact::Capture(v) => MatcherToken::Capture(v.capture_variant),
        }
    }
}

/// Produces a parser combinator that searches for the next possible set of strings of
/// characters used to terminate a forward search.
///
/// Take a peekable iterator.
/// Until a top level Match is encountered, peek through optional sections.
/// If a match is found, then move the list of delimiters into a take_till seeing if the current input slice is found in the list of decimeters.
/// If a match is not found, then do the same, or accept as part of an alt() a take the rest of the input (as long as it is valid).
/// return this take_till configuration and use that to terminate / capture the given string for the capture token.
pub fn next_delimiters<'a>(
    iter: Peekable<Iter<MatcherToken>>,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    enum MatchOrOptSequence<'a> {
        Match(&'a str),
    }

    let mut sequences = vec![];
    for next in iter {
        match next {
            MatcherToken::Exact(sequence) => {
                sequences.push(MatchOrOptSequence::Match(&sequence));
                break;
            }
            _ => panic!("underlying parser should not allow token order not of match or optional"),
        }
    }

    let delimiters: Vec<String> = sequences
        .into_iter()
        .map(|s| match s {
            MatchOrOptSequence::Match(s) => s,
        })
        .map(String::from)
        .collect();

    log::trace!(
        "delimiters in read_until_next_known_delimiter: {:?}",
        delimiters
    );

    // if the sequence contains an optional section, it can attempt to match until the end.
    map(alternative(delimiters), |x| x)
}



/// Parse the provided "matcher string" and then optimize the tokens.
//pub fn parse_str_and_optimize_tokens(i: &str) -> Result<Vec<MatcherToken>, YewRouterParseError> {
//    let tokens = parse(i)?;
//    Ok(optimize_tokens(tokens))
//}

pub use crate::parser2::parse_str_and_optimize_tokens;


//#[cfg(test)]
//mod test {
//    use super::*;
//    use crate::parser::CaptureVariant;
//
//    #[test]
//    fn conversion_cap_or_exact_to_matcher_token_exact() {
//        let mt = MatcherToken::from(CaptureOrExact::Exact("lorem".to_string()));
//        assert_eq!(mt, MatcherToken::Exact("lorem".to_string()))
//    }
//
//    #[test]
//    fn conversion_cap_or_exact_to_matcher_token_capture() {
//        use crate::Capture;
//        let mt = MatcherToken::from(CaptureOrExact::Capture(Capture::from(
//            CaptureVariant::Named("lorem".to_string()),
//        )));
//        assert_eq!(mt, MatcherToken::Capture(CaptureVariant::Named("lorem".to_string())))
//    }
//
//    #[test]
//    fn optimize_capture_all() {
//        use crate::Capture;
//        let tokens = vec![RouteParserToken::Capture(Capture::from(
//            CaptureVariant::ManyNamed("lorem".to_string()),
//        ))];
//        let optimized = optimize_tokens(tokens);
//        let expected = vec![MatcherToken::Capture(CaptureVariant::ManyNamed(
//            "lorem".to_string(),
//        ))];
//        assert_eq!(expected, optimized);
//    }
//
//    #[test]
//    fn optimize_capture_everything_after_initial_slash() {
//        use crate::Capture;
//        let tokens = vec![
//            RouteParserToken::Separator,
//            RouteParserToken::Capture(Capture::from(CaptureVariant::ManyNamed(
//                "lorem".to_string(),
//            ))),
//        ];
//        let optimized = optimize_tokens(tokens);
//        let expected = vec![
//            MatcherToken::Exact("/".to_string()),
//            MatcherToken::Capture(CaptureVariant::ManyNamed("lorem".to_string())),
//        ];
//        assert_eq!(expected, optimized);
//    }
//
//    #[test]
//    fn optimize_query_capture() {
//        use crate::Capture;
//        let tokens = vec![
//            RouteParserToken::QueryBegin,
//            RouteParserToken::QueryCapture {
//                ident: "lorem".to_string(),
//                capture_or_match: CaptureOrExact::Capture(Capture::from(CaptureVariant::Named("lorem".to_string()))),
//            },
//        ];
//        let optimized = optimize_tokens(tokens);
//        let expected = vec![
//            MatcherToken::Exact("?lorem=".to_string()),
//            MatcherToken::Capture(CaptureVariant::Named("lorem".to_string())),
//        ];
//        assert_eq!(expected, optimized);
//    }
//
//    #[test]
//    fn next_delimiter_simple() {
//        let tokens = vec![MatcherToken::Exact("/".to_string())];
//        next_delimiters(tokens.iter().peekable())("/").expect("should match");
//    }
//
//}
