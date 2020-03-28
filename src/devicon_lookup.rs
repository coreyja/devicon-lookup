use phf::phf_map;
use regex::Regex;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

const DEFAULT_SYMBOL: &'static str = "";
// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
static SYMBOL_MAP: phf::Map<&'static str, &'static str> = phf_map! {
        "ai" => "",
        "awk" => "",
        "bash" => "",
        "bat" => "",
        "bmp" => "",
        "c++" => "",
        "c" => "",
        "cc" => "",
        "clj" => "",
        "cljc" => "",
        "cljs" => "",
        "coffee" => "",
        "conf" => "",
        "cp" => "",
        "cpp" => "",
        "csh" => "",
        "css" => "",
        "cxx" => "",
        "d" => "",
        "dart" => "",
        "db" => "",
        "diff" => "",
        "dump" => "",
        "edn" => "",
        "ejs" => "",
        "erl" => "",
        "ex" => "",
        "eex" => "",
        "exs" => "",
        "f#" => "",
        "fish" => "",
        "fs" => "",
        "fsi" => "",
        "fsscript" => "",
        "fsx" => "",
        "gif" => "",
        "go" => "",
        "h" => "",
        "hbs" => "",
        "hpp" => "",
        "hrl" => "",
        "hs" => "",
        "htm" => "",
        "html" => "",
        "hxx" => "",
        "ico" => "",
        "ini" => "",
        "java" => "",
        "jl" => "",
        "jpeg" => "",
        "jpg" => "",
        "js" => "",
        "json" => "",
        "jsx" => "",
        "ksh" => "",
        "less" => "",
        "lhs" => "",
        "lua" => "",
        "markdown" => "",
        "md" => "",
        "ml" => "λ",
        "mli" => "λ",
        "mustache" => "",
        "php" => "",
        "pl" => "",
        "pm" => "",
        "png" => "",
        "pp" => "",
        "ps1" => "",
        "psb" => "",
        "psd" => "",
        "py" => "",
        "pyc" => "",
        "pyd" => "",
        "pyo" => "",
        "rb" => "",
        "rlib" => "",
        "rmd" => "",
        "rs" => "",
        "rss" => "",
        "sass" => "",
        "scala" => "",
        "scss" => "",
        "sh" => "",
        "slim" => "",
        "sln" => "",
        "sql" => "",
        "styl" => "",
        "suo" => "",
        "t" => "",
        "ts" => "",
        "tsx" => "",
        "twig" => "",
        "vi" => "",
        "vim" => "",
        "vue" => "﵂",
        "xul" => "",
        "yaml" => "",
        "yml" => "",
        "zsh" => "",
};

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
