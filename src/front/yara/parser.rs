//! Yara parser
#![allow(missing_docs)]

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

// Issue with the proc macro
/// Pest parser for our datalog
#[derive(Parser)]
#[grammar = "front/yara/grammar.pest"] // relative to src
struct MyParser;

pub mod ast {
    use super::Rule;
    use from_pest::ConversionError;
    use from_pest::FromPest;
    use from_pest::Void;
    use lazy_static::lazy_static;
    use pest::iterators::{Pair, Pairs};
    use pest::prec_climber::{Assoc, Operator, PrecClimber};
    pub use pest::Span;
    use pest_ast::FromPest;

    fn span_into_str(span: Span) -> &str {
        span.as_str()
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum Regex<'ast> {
        Empty,
        Char(char),
        Union(Box<Regex<'ast>>, Box<Regex<'ast>>),
        Concat(Box<Regex<'ast>>, Box<Regex<'ast>>),
        Finstar(Box<Regex<'ast>>, int)
    }

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::proposition))]
    pub enum Prop<'ast> {
        Bin(BinOp, Box<Prop<'ast>>, Box<Prop<'ast>>),
        Atom(Ident<'ast>),
        Pattern(Regex<'ast>)
    }

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::proposition_bin))]
    pub enum BinaryOperator {
        And,
        Or,
    }

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::str_pair))]
    pub struct NamedRegex<'ast> {
        pub name: &'ast str,
        pub expr: Regex<'ast>
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Rule<'ast> {
        pub span: Span<'ast>,
        pub strings: Vec<NamedRegex<'ast>>,
        pub condition: Vec<Prop<'ast>>
    }

    #[derive(Debug, FromPest, PartialEq, Clone)]
    #[pest_ast(rule(Rule::str_identifier))]
    pub struct Ident<'ast> {
        #[pest_ast(outer(with(span_into_str)))]
        pub value: &'ast str,
    }

    impl<'ast> FromPest<'ast> for Regex<'ast> {
        type Rule = Rule;
        type FatalError = Void;
        fn from_pest(pest: &mut Pairs<'ast, Rule>) -> Result<Self, ConversionError<Void>> {
            // get a clone to "try" to match
            let mut clone = pest.clone();
            // advance by one pair in the clone, if none error out, `pest` is still the original
            let pair = clone.next().ok_or(::from_pest::ConversionError::NoMatch)?;
            // this should be an expression
            match pair.as_rule() {
                Rule::regex => {
                    // we can replace `pest` with the clone we tried with and got pairs from to create the AST
                    *pest = clone;
                    Ok(*climb(pair))
                }
                _ => Err(ConversionError::NoMatch),
            }
        }
    }
}

pub fn parse(file_string: &str) -> Result<ast::Program, Error<Rule>> {
    let mut pest_pairs = MyParser::parse(Rule::program, file_string)?;
    use from_pest::FromPest;
    Ok(ast::Program::from_pest(&mut pest_pairs).expect("bug in AST construction"))
}
