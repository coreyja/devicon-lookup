use phf::phf_map;
use regex::Regex;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

const DEFAULT_SYMBOL: &'static str = "";
const ERROR_SYMBOL: &'static str = "";
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
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
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

pub fn strip_color(input: ParserResult) -> ParserResult {
    Ok(ANSI_COLOR_REGEX.replace_all(&input?, "").to_string())
}
