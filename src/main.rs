#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{self, BufRead};
use std::path::Path;

static DEFAULT_SYMBOL: &str = & "";
lazy_static! {
	static ref SYMBOL_MAP: HashMap<&'static str, &'static str> = {
		let mut m = HashMap::new();
		m.insert("ai", "");
		m.insert("awk", "");
		m.insert("bash", "");
		m.insert("bat", "");
		m.insert("bmp", "");
		m.insert("c++", "");
		m.insert("c", "");
		m.insert("cc", "");
		m.insert("clj", "");
		m.insert("cljc", "");
		m.insert("cljs", "");
		m.insert("coffee", "");
		m.insert("conf", "");
		m.insert("cp", "");
		m.insert("cpp", "");
		m.insert("csh", "");
		m.insert("css", "");
		m.insert("cxx", "");
		m.insert("d", "");
		m.insert("dart", "");
		m.insert("db", "");
		m.insert("diff", "");
		m.insert("dump", "");
		m.insert("edn", "");
		m.insert("ejs", "");
		m.insert("erl", "");
		m.insert("f#", "");
		m.insert("fish", "");
		m.insert("fs", "");
		m.insert("fsi", "");
		m.insert("fsscript", "");
		m.insert("fsx", "");
		m.insert("gif", "");
		m.insert("go", "");
		m.insert("h", "");
		m.insert("hbs", "");
		m.insert("hpp", "");
		m.insert("hrl", "");
		m.insert("hs", "");
		m.insert("htm", "");
		m.insert("html", "");
		m.insert("hxx", "");
		m.insert("ico", "");
		m.insert("ini", "");
		m.insert("java", "");
		m.insert("jl", "");
		m.insert("jpeg", "");
		m.insert("jpg", "");
		m.insert("js", "");
		m.insert("json", "");
		m.insert("jsx", "");
		m.insert("ksh", "");
		m.insert("less", "");
		m.insert("lhs", "");
		m.insert("lua", "");
		m.insert("markdown", "");
		m.insert("md", "");
		m.insert("ml", "λ");
		m.insert("mli", "λ");
		m.insert("mustache", "");
		m.insert("php", "");
		m.insert("pl", "");
		m.insert("pm", "");
		m.insert("png", "");
		m.insert("pp", "");
		m.insert("ps1", "");
		m.insert("psb", "");
		m.insert("psd", "");
		m.insert("py", "");
		m.insert("pyc", "");
		m.insert("pyd", "");
		m.insert("pyo", "");
		m.insert("rb", "");
		m.insert("rlib", "");
		m.insert("rmd", "");
		m.insert("rs", "");
		m.insert("rss", "");
		m.insert("sass", "");
		m.insert("scala", "");
		m.insert("scss", "");
		m.insert("sh", "");
		m.insert("slim", "");
		m.insert("sln", "");
		m.insert("sql", "");
		m.insert("styl", "");
		m.insert("suo", "");
		m.insert("t", "");
		m.insert("ts", "");
		m.insert("tsx", "");
		m.insert("twig", "");
		m.insert("vim", "");
		m.insert("vim", "");
		m.insert("vue", "﵂");
		m.insert("xul", "");
		m.insert("yaml", "");
		m.insert("yml", "");
		m.insert("zsh", "");

		m
	};
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
			Some(extension) => SYMBOL_MAP.get(& extension).unwrap_or(& DEFAULT_SYMBOL),
			None => DEFAULT_SYMBOL
		};
		println!("{} {}", symbol, filename);
	}
}

