use phf::phf_map;
use regex::Regex;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;

mod map;
pub mod parsers;

pub const DEFAULT_SYMBOL: &'static str = "";
pub const ERROR_SYMBOL: &'static str = "";

pub type ParserResult = Result<String, &'static str>;
pub type Parser = dyn Fn(ParserResult) -> ParserResult;

pub struct ParsedLine {
    original: String,
    filename: String,
}

pub struct Line<'a> {
    pub original: String,
    pub filename_parsers: Vec<&'a Parser>,
}

pub struct LineBuilder<'a> {
    original: String,
    filename_parsers: Vec<&'a Parser>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(original: String) -> Self {
        Self {
            original: original,
            filename_parsers: vec![],
        }
    }

    pub fn with_parser(&mut self, parser: &'a Parser) {
        self.filename_parsers.push(parser);
    }

    pub fn build(self) -> Line<'a> {
        Line {
            original: self.original,
            filename_parsers: self.filename_parsers,
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(self) -> Result<ParsedLine, &'static str> {
        let mut curr: ParserResult = Ok(self.original.clone());

        for parser in self.filename_parsers.iter() {
            curr = parser(curr)
        }

        curr.and_then(|x| {
            Ok(ParsedLine {
                original: self.original,
                filename: x,
            })
        })
    }
}

impl ParsedLine {
    fn extension(&self) -> Option<&str> {
        Path::new(&self.filename)
            .extension()
            .and_then(OsStr::to_str)
    }

    fn symbol(&self) -> &str {
        match self.extension() {
            Some(extension) => map::find_symbol(extension),
            None => &DEFAULT_SYMBOL,
        }
    }

    pub fn print_with_symbol(&self) {
        match writeln!(&mut io::stdout(), "{} {}", self.symbol(), self.original) {
            Ok(_) => (),
            Err(_) => ::std::process::exit(0),
        }
    }
}
