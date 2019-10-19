//! Parser for a "matcher string". The tokens produced by this parser are used to construct a matcher.

#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_qualifications
)]

//pub mod parser_old;
pub mod parser;
mod token_optimizer;

//pub use parser_old::{Capture, CaptureVariant};
use std::collections::HashMap;
pub use token_optimizer::{
    parse_str_and_optimize_tokens, MatcherToken, CaptureVariant
};
pub use parser::ParserError;

/// Captures contain keys corresponding to named match sections,
/// and values containing the content captured by those sections.
pub type Captures<'a> = HashMap<&'a str, String>;
