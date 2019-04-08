use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

const DEFAULT_SYMBOL: &'static str = "";

pub fn strip_ansi_codes(input: &str) -> String {
    ANSI_COLOR_REGEX.replace_all(input, "").to_string()
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub trait Symbolable {
    fn to_parse(&self) -> &str;
    fn to_print(&self) -> &str;

    fn extension(&self) -> Option<&str> {
        Path::new(self.to_parse()).extension().and_then(OsStr::to_str)
    }

    fn symbol(&self) -> &str {
        match self.extension() {
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
            None => &DEFAULT_SYMBOL,
        }
    }

    fn print_with_symbol(&self) {
        println!("{} {}", self.symbol(), self.to_print());
    }
}

pub struct Line {
    line: String,
}

impl Line {
    fn new(line: String) -> Line {
        Line { line }
    }

    pub fn boxed(line: String) -> Box<Line> {
        Box::new(Line::new(line))
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
    fn new(line: String) -> ColoredLine {
        let stripped_line = strip_ansi_codes(&line);
        ColoredLine {
            line,
            stripped_line: stripped_line,
        }
    }

    pub fn boxed(line: String) -> Box<ColoredLine> {
        Box::new(ColoredLine::new(line))
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