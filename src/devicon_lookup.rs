use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

static DEFAULT_SYMBOL: &str = &"";

lazy_static! {
    static ref ANSI_COLOR_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn get_symbol_from_extension(extension: &str) -> &str {
    SYMBOL_MAP.get(extension).unwrap_or(&DEFAULT_SYMBOL)
}

pub fn strip_ansi_codes(input: &str) -> String {
    ANSI_COLOR_REGEX.replace_all(input, "").to_string()
}

pub fn get_default_symbol() -> &'static str {
    DEFAULT_SYMBOL
}
