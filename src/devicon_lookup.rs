use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;
use std::io::{self, Write};

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

const DEFAULT_SYMBOL: &'static str = "";
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub trait Symbolable {
    fn to_parse(&self) -> &str;
    fn to_print(&self) -> &str;

    fn extension(&self) -> Option<&str> {
        Path::new(self.to_parse())
            .extension()
            .and_then(OsStr::to_str)
    }

    fn symbol(&self) -> &str {
        match self.extension() {
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
            None => &DEFAULT_SYMBOL,
        }
    }

    fn print_with_symbol(&self) {
        match writeln!(&mut io::stdout(), "{} {}", self.symbol(), self.to_print()) {
            Ok(_) => (),
            Err(_) => ::std::process::exit(0),
        }
    }
}

pub struct Line {
    line: String,
}

impl Line {
    fn new(line: String) -> Line {
        Line { line }
    }

    pub fn boxed(line: String) -> Box<Self> {
        Box::new(Self::new(line))
    }
}

impl Symbolable for Line {
    fn to_parse(&self) -> &str {
        &self.line
    }
    fn to_print(&self) -> &str {
        &self.line
    }
}

pub struct ColoredLine {
    line: String,
    stripped_line: String,
}

impl ColoredLine {
    fn strip_ansi_codes(input: &str) -> String {
        ANSI_COLOR_REGEX.replace_all(input, "").to_string()
    }

    fn new(line: String) -> ColoredLine {
        let stripped_line = Self::strip_ansi_codes(&line);
        ColoredLine {
            line,
            stripped_line: stripped_line,
        }
    }

    pub fn boxed(line: String) -> Box<Self> {
        Box::new(Self::new(line))
    }
}

impl Symbolable for ColoredLine {
    fn to_parse(&self) -> &str {
        &self.stripped_line
    }
    fn to_print(&self) -> &str {
        &self.line
    }
}
