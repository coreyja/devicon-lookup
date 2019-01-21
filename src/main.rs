use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;

const SYMBOLS: [(&str, &str); 97] = [
  ("ai", ""),
  ("awk", ""),
  ("bash", ""),
  ("bat", ""),
  ("bmp", ""),
  ("c", ""),
  ("c++", ""),
  ("cc", ""),
  ("clj", ""),
  ("cljc", ""),
  ("cljs", ""),
  ("coffee", ""),
  ("conf", ""),
  ("cp", ""),
  ("cpp", ""),
  ("csh", ""),
  ("css", ""),
  ("cxx", ""),
  ("d", ""),
  ("dart", ""),
  ("db", ""),
  ("diff", ""),
  ("dump", ""),
  ("edn", ""),
  ("ejs", ""),
  ("erl", ""),
  ("f#", ""),
  ("fish", ""),
  ("fs", ""),
  ("fsi", ""),
  ("fsscript", ""),
  ("fsx", ""),
  ("gif", ""),
  ("go", ""),
  ("h", ""),
  ("hbs", ""),
  ("hpp", ""),
  ("hrl", ""),
  ("hs", ""),
  ("htm", ""),
  ("html", ""),
  ("hxx", ""),
  ("ico", ""),
  ("ini", ""),
  ("java", ""),
  ("jl", ""),
  ("jpeg", ""),
  ("jpg", ""),
  ("js", ""),
  ("json", ""),
  ("jsx", ""),
  ("ksh", ""),
  ("less", ""),
  ("lhs", ""),
  ("lua", ""),
  ("markdown", ""),
  ("md", ""),
  ("ml", "λ"),
  ("mli", "λ"),
  ("mustache", ""),
  ("php", ""),
  ("pl", ""),
  ("pm", ""),
  ("png", ""),
  ("pp", ""),
  ("ps1", ""),
  ("psb", ""),
  ("psd", ""),
  ("py", ""),
  ("pyc", ""),
  ("pyd", ""),
  ("pyo", ""),
  ("rb", ""),
  ("rlib", ""),
  ("rmd", ""),
  ("rs", ""),
  ("rss", ""),
  ("sass", ""),
  ("scala", ""),
  ("scss", ""),
  ("sh", ""),
  ("slim", ""),
  ("sln", ""),
  ("sql", ""),
  ("styl", ""),
  ("suo", ""),
  ("t", ""),
  ("ts", ""),
  ("tsx", ""),
  ("twig", ""),
  ("vim", ""),
  ("vim", ""),
  ("vue", "﵂"),
  ("xul", ""),
  ("yaml", ""),
  ("yml", ""),
  ("zsh", ""),
  ];
const DEFAULT_SYMBOL: &str = "Z";

fn get_extension_from_filename(filename: &str) -> Option<&str> {
	Path::new(filename)
		.extension()
		.and_then(OsStr::to_str)
}

fn get_symbol_from_extension(extension: &str) -> &str {
  let index = SYMBOLS.binary_search_by_key(&extension, |&(ext, _sym)| ext);
  let symbol = match index {
    Ok(i) => SYMBOLS[i].1,
    Err(_index) => DEFAULT_SYMBOL,
  };

  return symbol;
}

fn main() {
	let stdin = io::stdin();
	for line in stdin.lock().lines() {
		let filename = line.unwrap();
		let extension = get_extension_from_filename(& filename);
		let symbol = match extension {
			Some(ext) => get_symbol_from_extension(& ext),
			None => DEFAULT_SYMBOL
		};
		println!("{} {}", symbol, filename);
	}
}

