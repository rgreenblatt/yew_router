//use crate::parser::parse;
//use crate::parser::RouteParserToken;
//use crate::parser::{CaptureOrExact};
use nom::IResult;
use std::iter::Peekable;
use std::slice::Iter;
use nom::combinator::{map};

/// Tokens used to determine how to match and capture sections from a URL.
#[derive(Debug, PartialEq, Clone)]
pub enum MatcherToken {
    /// Section-related tokens can be condensed into a match.
    Exact(String),
    /// Capture section.
    Capture(CaptureVariant),
}

/// Variants that indicate how part of a string should be captured.
#[derive(Debug, PartialEq, Clone)]
pub enum CaptureVariant {
    /// {name} - captures a section and adds it to the map with a given name.
    Named(String),
    /// {*:name} - captures over many sections and adds it to the map with a given name.
    ManyNamed(String),
    /// {2:name} - captures a fixed number of sections with a given name.
    NumberedNamed {
        /// Number of sections to match.
        sections: usize,
        /// The key to be entered in the `Matches` map.
        name: String,
    },
}


pub use crate::parser::parse_str_and_optimize_tokens;


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
