//! Charabia library tokenize a text detecting the Script/Language, segmenting, normalizing, and classifying it.
//!
//! Examples
//! --------
//! #### Tokenization
//! ```
//! use charabia::Tokenize;
//!
//! let orig = "Thé quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
//!
//! // tokenize the text.
//! let mut tokens = orig.tokenize();
//!
//! let token = tokens.next().unwrap();
//! // the lemma into the token is normalized: `Thé` became `the`.
//! assert_eq!(token.lemma(), "the");
//! // token is classfied as a word
//! assert!(token.is_word());
//!
//! let token = tokens.next().unwrap();
//! assert_eq!(token.lemma(), " ");
//! // token is classfied as a separator
//! assert!(token.is_separator());
//! ```
//!
//! #### Segmentation
//! ```
//! use charabia::Segment;
//!
//! let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
//!
//! let mut segments = orig.segment_str();
//!
//! assert_eq!(segments.next(), Some("The"));
//! assert_eq!(segments.next(), Some(" "));
//! assert_eq!(segments.next(), Some("quick"));
//! ```
//!
//! Build features
//! --------
//! Charabia comes with default features that can be deactivated at compile time,
//! this features are additional Language supports that need to download and/or build a specialized dictionary that impact the compilation time.
//! Theses features are listed in charabia's `cargo.toml` and can be deactivated via [dependency features](https://doc.rust-lang.org/cargo/reference/features.html#dependency-features).

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod normalizer;
pub mod segmenter;
pub mod separators;

mod detection;
mod token;
mod tokenizer;

pub use detection::{Language, Script, StrDetection};
pub use normalizer::Normalize;
pub use segmenter::Segment;
pub use token::{SeparatorKind, Token, TokenKind};

#[cfg(test)]
pub use token::StaticToken;

pub use crate::tokenizer::{ReconstructedTokenIter, Tokenize, Tokenizer, TokenizerBuilder};
