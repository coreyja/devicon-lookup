#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;

static DEFAULT_SYMBOL: &str = & "";

const SYMBOL_COUNT: usize = 97;
const SYMBOLS: [[&str; 2]; SYMBOL_COUNT] = [
    ["ai", ""],
    ["awk", ""],
    ["bash", ""],
    ["bat", ""],
		["bmp", ""],
		["c++", ""],
		["c", ""],
		["cc", ""],
		["clj", ""],
		["cljc", ""],
		["cljs", ""],
		["coffee", ""],
		["conf", ""],
		["cp", ""],
		["cpp", ""],
		["csh", ""],
		["css", ""],
		["cxx", ""],
		["d", ""],
		["dart", ""],
		["db", ""],
		["diff", ""],
		["dump", ""],
		["edn", ""],
		["ejs", ""],
		["erl", ""],
		["f#", ""],
		["fish", ""],
		["fs", ""],
		["fsi", ""],
		["fsscript", ""],
		["fsx", ""],
		["gif", ""],
		["go", ""],
		["h", ""],
		["hbs", ""],
		["hpp", ""],
		["hrl", ""],
		["hs", ""],
		["htm", ""],
		["html", ""],
		["hxx", ""],
		["ico", ""],
		["ini", ""],
		["java", ""],
		["jl", ""],
		["jpeg", ""],
		["jpg", ""],
		["js", ""],
		["json", ""],
		["jsx", ""],
		["ksh", ""],
		["less", ""],
		["lhs", ""],
		["lua", ""],
		["markdown", ""],
		["md", ""],
		["ml", "λ"],
		["mli", "λ"],
		["mustache", ""],
		["php", ""],
		["pl", ""],
		["pm", ""],
		["png", ""],
		["pp", ""],
		["ps1", ""],
		["psb", ""],
		["psd", ""],
		["py", ""],
		["pyc", ""],
		["pyd", ""],
		["pyo", ""],
		["rb", ""],
		["rlib", ""],
		["rmd", ""],
		["rs", ""],
		["rss", ""],
		["sass", ""],
		["scala", ""],
		["scss", ""],
		["sh", ""],
		["slim", ""],
		["sln", ""],
		["sql", ""],
		["styl", ""],
		["suo", ""],
		["t", ""],
		["ts", ""],
		["tsx", ""],
		["twig", ""],
		["vim", ""],
		["vim", ""],
		["vue", "﵂"],
		["xul", ""],
		["yaml", ""],
		["yml", ""],
		["zsh", ""]
];

fn binary_search(extension: &str, start: usize, end: usize) -> Option<&str> {
  let cur = (start + end) / 2;
  let current_extention = SYMBOLS[cur][0];

  if current_extention == extension {
    return Some(SYMBOLS[cur][1]);
  } else {
    if current_extention < extension {
      return binary_search(extension, start, cur);
    } else {
      return binary_search(extension, cur, end);
    }
  }
}

fn get_symbol_from_extenstion(extension: &str) -> Option<&str> {
  let num = SYMBOLS.binary_search_by_key(&extension, |x| x[0]).ok();

  print!("{num}", num=num);
  return SYMBOLS[num][1];
  // return binary_search(extension, 0, SYMBOL_COUNT);
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
	Path::new(filename)
		.extension()
		.and_then(OsStr::to_str)
}

fn main() {
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let filename = line.unwrap();
		let extension = get_extension_from_filename(& filename);
		let symbol = match extension {
			Some(extension) => get_symbol_from_extenstion(extension).unwrap_or(& DEFAULT_SYMBOL),
			None => DEFAULT_SYMBOL
		};
		println!("{} {}", symbol, filename);
	}
}

