#[macro_use] extern crate lazy_static;
extern crate phf;

use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

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

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let filename = line.unwrap();
        let decolored_filename = strip_ansi_codes(&filename);
        let extension = get_extension_from_filename(&decolored_filename);
        let symbol = match extension {
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
            None => DEFAULT_SYMBOL,
        };
        println!("{} {}", symbol, filename);
    }
}
