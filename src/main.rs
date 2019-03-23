extern crate phf;

use colored::*;
use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

static DEFAULT_SYMBOL: &str = &"";

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let filename = line.unwrap();
        let decolored_filename = filename.normal().clear();
        let extension = get_extension_from_filename(&decolored_filename);
        let symbol = match extension {
            Some(extension) => SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL),
            None => DEFAULT_SYMBOL,
        };
        println!("{} {}", symbol, filename);
    }
}
