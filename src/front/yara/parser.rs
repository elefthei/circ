//! Yara parser
#![allow(missing_docs)]

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

/// Pest parser for our yara language
#[derive(Parser)]
#[grammar = "front/yara/grammar.pest"] // relative to src
struct YaraParser;

fn main() {
}
