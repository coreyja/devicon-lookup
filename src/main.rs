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

fn strip_ansi_codes(input: &str) -> String {
    lazy_static! {
        static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    }
        ANSI_COLOR_REGEX.replace_all(input, "").to_string()
}

fn maybe_strip_color(strip_color: bool, input: &str) -> String {
    if strip_color {
        strip_ansi_codes(input)
    } else {
        String::from(input)
    }
}

fn process_stdin(strip_color: bool) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let filename = line.unwrap();
        let file_name_to_parse = maybe_strip_color(strip_color, &filename);
        let extension = get_extension_from_filename(&file_name_to_parse);
        let symbol = match extension {
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
            None => DEFAULT_SYMBOL,
        };
        println!("{} {}", symbol, filename);
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
