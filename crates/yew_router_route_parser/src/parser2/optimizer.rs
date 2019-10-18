use crate::parser2::{RouteParserToken, CaptureVariant, CaptureOrExact, parse, ParserError};

use crate::token_optimizer::MatcherToken;

impl <'a> From<CaptureVariant<'a>> for crate::parser::CaptureVariant {
    fn from(v: CaptureVariant<'a>) -> Self {
        match v {
            CaptureVariant::Named(s) => crate::parser::CaptureVariant::Named(s.to_string()),
            CaptureVariant::ManyNamed(s) => crate::parser::CaptureVariant::ManyNamed(s.to_string()),
            CaptureVariant::NumberedNamed { sections, name } => crate::parser::CaptureVariant::NumberedNamed {sections, name: name.to_string()}
        }
    }
}

impl <'a> From<CaptureOrExact<'a>> for MatcherToken {
    fn from(value: CaptureOrExact<'a>) -> Self {
        match value {
            CaptureOrExact::Exact(m) => MatcherToken::Exact(m.to_string()),
            CaptureOrExact::Capture(v) => MatcherToken::Capture(v.into()),
        }
    }
}

impl <'a> RouteParserToken<'a> {
    fn as_str(&self) -> &str {
        match self {
            RouteParserToken::Separator => "/",
            RouteParserToken::Exact(literal) => &literal,
            RouteParserToken::QueryBegin => "?",
            RouteParserToken::QuerySeparator => "&",
            RouteParserToken::FragmentBegin => "#",
            RouteParserToken::Capture { .. }
            | RouteParserToken::QueryCapture { .. }
            | RouteParserToken::End
            => unreachable!(),
        }
    }
}


/// Parse the provided "matcher string" and then optimize the tokens.
pub fn parse_str_and_optimize_tokens(i: &str) -> Result<Vec<MatcherToken>, (&str, ParserError)> {
    let tokens = parse(i)?;
    Ok(convert_tokens(&tokens))
}

/// Converts a slice of `RouteParserToken` into a Vec of MatcherTokens.
fn convert_tokens(tokens: &[RouteParserToken]) -> Vec<MatcherToken> {
    let mut new_tokens = vec![];
    let mut run: Vec<RouteParserToken> = vec![];

    let mut token_iter = tokens.iter();

    while let Some(token) = token_iter.next() {
        match token {
            RouteParserToken::QueryBegin
            | RouteParserToken::FragmentBegin
            | RouteParserToken::Separator
            | RouteParserToken::QuerySeparator
            | RouteParserToken::Exact(_)
            => {
                run.push(*token)
            }
            RouteParserToken::Capture(cap) => {
                new_tokens.push(MatcherToken::Exact(run.iter().map(RouteParserToken::as_str).collect()));
                run = vec![];
                new_tokens.push(MatcherToken::Capture(crate::parser::CaptureVariant::from(*cap)))
            }
            RouteParserToken::QueryCapture { ident, capture_or_match } => {
                match capture_or_match {
                    CaptureOrExact::Exact(s) => {
                        run.push(RouteParserToken::Exact(ident));
                        run.push(RouteParserToken::Exact("="));
                        run.push(RouteParserToken::Exact(s));
                    }
                    CaptureOrExact::Capture(cap) => {
                        let sequence = run.iter()
                            .map(RouteParserToken::as_str)
                            .chain(Some(*ident))
                            .chain(Some("="))
                            .collect();
                        new_tokens.push(MatcherToken::Exact(sequence));
                        run = vec![];
                        new_tokens.push(MatcherToken::Capture(crate::parser::CaptureVariant::from(*cap)))
                    }
                }

            }
            RouteParserToken::End => unimplemented!()
        }
    }

    if !run.is_empty(){
        new_tokens.push(MatcherToken::Exact(run.iter().map(RouteParserToken::as_str).collect()));
    }

    new_tokens
}

