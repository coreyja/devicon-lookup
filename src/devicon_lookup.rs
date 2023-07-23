use std::io::{self, Write};
use std::path::Path;

mod map;
pub mod parsers;

pub const DEFAULT_SYMBOL: &str = "";

pub trait ParserFn: Fn(String) -> ParserResult {}

impl<T: Fn(String) -> ParserResult> ParserFn for T {}

pub type ParserResult = Result<String, &'static str>;
pub type Parser = dyn ParserFn;

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
            original,
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
            curr = parser(curr?)
        }

        let filename = curr?;

        Ok(ParsedLine {
            original: self.original,
            filename,
        })
    }
}

impl ParsedLine {
    fn extension(&self) -> Option<&str> {
        Path::new(&self.filename).extension()?.to_str()
    }

    fn symbol(&self) -> &str {
        match self.extension() {
            Some(extension) => map::find_symbol(extension),
            None => DEFAULT_SYMBOL,
        }
    }

    pub fn print_with_symbol(&self) {
        let s = format!("{} {}\n", self.symbol(), self.original);
        write_to_stdout(s.as_bytes())
    }
}

pub fn write_to_stdout(item: &[u8]) {
    let write_result = io::stdout().write_all(item);

    if write_result.is_err() {
        ::std::process::exit(0)
    }
}
