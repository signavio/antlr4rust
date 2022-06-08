#![crate_type = "lib"]
#![feature(try_blocks)]
//#![feature(nll)]
#![feature(is_sorted)]
#![feature(cell_update)]
#![feature(get_mut_unchecked)]
#![feature(specialization)]
#![feature(coerce_unsized)]
#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
// #![feature(generic_associated_types)]
#![warn(rust_2018_idioms)]
#![warn(missing_docs)] // warn if there is missing docs
#![warn(trivial_numeric_casts)]
#![allow(incomplete_features)]

//! # Antlr4 runtime
//!
//! This is a Rust runtime for [ANTLR4] parser generator.
//! It is required to use parsers and lexers generated by [ANTLR4] parser generator
//!
//! This documentation refers to particular api used by generated parsers,lexers and syntax trees.
//!
//! For info on what is [ANTLR4] and how to generate parser please refer to:
//!  - [ANTLR4] main repository
//!  - [README] for Rust target
//!
//! [ANTLR4]: https://github.com/antlr/antlr4
//! [README]: https://github.com/rrevenantt/antlr4rust/blob/master/README.md
//!
//! ### Customization
//!
//! All input and output can be customized and optimized for particular usecase by implementing
//! related trait. Each of them already has different implementations that should be enough for most cases.
//! For more details see docs for corresponding trait and containing module.
//!
//! Currently available are:
//!  - [`CharStream`] - Lexer input, stream of char values with slicing support
//!  - [`TokenFactory`] - How lexer creates tokens.
//!  - [`Token`] - Element of [`TokenStream`]
//!  - [`TokenStream`] - Parser input, created from lexer or other token source.
//!  - [`ParserRuleContext`] - Node of created syntax tree.
//!
//! ### Zero-copy and lifetimes
//!
//! This library supports full zero-copy parsing. To allow this
//! `'input` lifetime is used everywhere inside to refer to data borrowed by parser/lexer.
//! Besides references to input it also can be [`TokenFactory`] if it returns references to tokens.
//! See [`ArenaFactory`] as an example of such behavior.
//! It allocates tokens in [`Arena`](typed_arena::Arena) and returns references.
//!
//! Using generated parse tree you should be careful to not require longer lifetime after the parsing.
//! If that's the case you will likely get "does not live long enough" error on the input string,
//! despite actual lifetime conflict is happening much later
//!
//! If you need to generate owned versions of parse tree or you want simpler usage,
//! you can opt out zero-copy by requiring `'input` to be static. In this case it is easier to also use
//! types that contains "owned" in their name or constructor function like `OwningTokenFactory`
//! or `InputStream::new_owned()`.
//!
//! ### Visitors and Listeners
//!
//! Parse listeners must outlive `'input` because they have to be stored inside of the parser.
//! It still allows to retrieve borrowed data from parse tree which should be enough to cover 99% use cases.
//!
//! `ParseTreeWalker` can accept listeners with arbitrary lifetime.
//!
//! `Visitor`s also can have arbitrary lifetime.
//!
//! ### Downcasting
//!
//! Rule context trait object support downcasting even for zero-copy case.
//! Also generic types(currently these are `H:ErrorStrategy` and `I:`[`TokenStream`]) that you can
//! access in generated parser from embedded actions also can be downcasted to concrete types.
//! To do it `TidExt::downcast_*` extension methods should be used.
//!
//! [`CharStream`]: crate::char_stream::CharStream
//! [`TokenFactory`]: crate::token_factory::TokenFactory
//! [`ArenaFactory`]: crate::token_factory::ArenaFactory
//! [`Token`]: crate::token::Token
//! [`TokenStream`]: crate::token_stream::TokenStream
//! [`ParserRuleContext`]: crate::parser_rule_context::ParserRuleContext

#[macro_use]
extern crate lazy_static;

#[doc(hidden)]
pub use lazy_static::lazy_static;

#[doc(hidden)]
pub use parking_lot::RwLock;

#[doc(hidden)]
pub use better_any::{impl_tid, type_id, Tid, TidAble, TidExt};

#[doc(inline)]
pub use error_strategy::{BailErrorStrategy, DefaultErrorStrategy, ErrorStrategy};

pub use input_stream::InputStream;

#[doc(inline)]
pub use lexer::{BaseLexer, Lexer};
#[doc(inline)]
pub use parser::{BaseParser, ListenerId, Parser};
#[doc(inline)]
pub use token_source::TokenSource;
//extern crate uuid;
#[doc(hidden)]
pub use prediction_context::PredictionContextCache;

#[doc(inline)]
pub use prediction_mode::PredictionMode;

#[doc(hidden)]
pub mod atn_config;
#[doc(hidden)]
pub mod atn_simulator;
pub mod int_stream;
mod lexer_action;
mod ll1_analyzer;
#[doc(hidden)]
pub mod recognizer;
pub mod token_factory;
//pub mod tokenstream_rewriter;
#[doc(hidden)]
pub mod atn_deserialization_options;
#[doc(hidden)]
pub mod atn_state;
pub mod char_stream;
#[doc(hidden)]
pub mod dfa_state;
#[doc(hidden)]
pub mod interval_set;
pub mod parser_rule_context;
mod prediction_context;
#[doc(hidden)]
pub mod semantic_context;
mod token_source;
pub mod token_stream;
//pub mod trace_listener;
#[doc(hidden)]
pub mod dfa;
#[doc(hidden)]
pub mod transition;
pub mod tree;
//pub mod file_stream;
#[doc(hidden)]
pub mod atn;
#[doc(hidden)]
pub mod atn_config_set;
#[doc(hidden)]
pub mod atn_deserializer;
pub mod common_token_stream;
mod dfa_serializer;
pub mod error_listener;
pub mod error_strategy;
pub mod errors;
pub mod input_stream;
pub mod lexer;
#[doc(hidden)]
pub mod lexer_action_executor;
pub mod lexer_atn_simulator;
pub mod parser;
pub mod parser_atn_simulator;
mod prediction_mode;
pub mod token;
pub mod trees;
mod utils;
//pub mod tokenstream_rewriter_test;
mod atn_type;
// mod context_factory;
pub mod rule_context;
pub mod vocabulary;
//#[cfg(test)]
// tests are either integration tests in "tests" foulder or unit tests in some modules
