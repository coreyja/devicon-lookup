#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use docopt::Docopt;
use regex::Regex;
use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const USAGE: &'static str = include_str!("USAGE.txt");

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

#[derive(Debug, Deserialize)]
struct Args {
    flag_color: bool,
    flag_version: bool,
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

static DEFAULT_SYMBOL: &str = &"";

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn get_symbol_from_extension(extension: &str) -> &str {
    SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL)
}

fn strip_ansi_codes(input: &str) -> String {
    ANSI_COLOR_REGEX.replace_all(input, "").to_string()
}

fn process_line(strip_color: bool, line: &str) {
    let filename = String::from(line);
    let file_name_to_parse = if strip_color {
        strip_ansi_codes(&filename)
    } else {
        filename.clone()
    };
    let extension = get_extension_from_filename(&file_name_to_parse);
    let symbol = match extension {
        Some(extension) => get_symbol_from_extension(extension),
        None => DEFAULT_SYMBOL,
    };
    println!("{} {}", symbol, filename);
}

fn process_stdin(strip_color: bool) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        process_line(strip_color, &line.unwrap());
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("Dev Icon Lookup v{}", VERSION);
    } else {
        process_stdin(args.flag_color);
    }
}
