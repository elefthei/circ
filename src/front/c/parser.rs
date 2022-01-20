//! Parsing and recursively loading C.

use lang_c::driver::Error;
use lang_c::driver::{parse, Config, Parse};
use rug::Integer;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
<<<<<<< HEAD
=======
use std::path::Path;
>>>>>>> 75572c6... C Frontend (#22)
use std::path::PathBuf;

/// Parse an inputs file where each line has format: `no-whitespace integer`.
///
/// Permits blank lines and ignores non-separating whitespace.
///
/// ```ignore
/// x 5
/// x.y -7
/// ```
pub fn parse_inputs(p: PathBuf) -> HashMap<String, Integer> {
    let mut m = HashMap::new();
    for l in BufReader::new(File::open(p).unwrap()).lines() {
        let l = l.unwrap();
        let l = l.trim();
<<<<<<< HEAD
        if l.len() > 0 {
=======
        if !l.is_empty() {
>>>>>>> 75572c6... C Frontend (#22)
            let mut s = l.split_whitespace();
            let key = s.next().unwrap().to_owned();
            let value = Integer::from(Integer::parse_radix(&s.next().unwrap(), 10).unwrap());
            m.insert(key, value);
        }
    }
    m
}

pub struct CParser {
    config: Config,
}

impl CParser {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

<<<<<<< HEAD
    pub fn parse_file(&self, path: &PathBuf) -> Result<Parse, Error> {
        Ok(parse(&self.config, path)?)
=======
    pub fn parse_file(&self, path: &Path) -> Result<Parse, Error> {
        parse(&self.config, path)
>>>>>>> 75572c6... C Frontend (#22)
    }
}
