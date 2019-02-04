extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
  let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
  let mut file = BufWriter::new(File::create(&path).unwrap());

  write!(&mut file, "static SYMBOL_MAP: phf::Map<&'static str, &'static str> = ").unwrap();
  phf_codegen::Map::new()
    .entry("ai", "\"\"")
    .entry("awk", "\"\"")
    .entry("bash", "\"\"")
    .entry("bat", "\"\"")
    .entry("bmp", "\"\"")
    .entry("c++", "\"\"")
    .entry("c", "\"\"")
    .entry("cc", "\"\"")
    .entry("clj", "\"\"")
    .entry("cljc", "\"\"")
    .entry("cljs", "\"\"")
    .entry("coffee", "\"\"")
    .entry("conf", "\"\"")
    .entry("cp", "\"\"")
    .entry("cpp", "\"\"")
    .entry("csh", "\"\"")
    .entry("css", "\"\"")
    .entry("cxx", "\"\"")
    .entry("d", "\"\"")
    .entry("dart", "\"\"")
    .entry("db", "\"\"")
    .entry("diff", "\"\"")
    .entry("dump", "\"\"")
    .entry("edn", "\"\"")
    .entry("ejs", "\"\"")
    .entry("erl", "\"\"")
    .entry("f#", "\"\"")
    .entry("fish", "\"\"")
    .entry("fs", "\"\"")
    .entry("fsi", "\"\"")
    .entry("fsscript", "\"\"")
    .entry("fsx", "\"\"")
    .entry("gif", "\"\"")
    .entry("go", "\"\"")
    .entry("h", "\"\"")
    .entry("hbs", "\"\"")
    .entry("hpp", "\"\"")
    .entry("hrl", "\"\"")
    .entry("hs", "\"\"")
    .entry("htm", "\"\"")
    .entry("html", "\"\"")
    .entry("hxx", "\"\"")
    .entry("ico", "\"\"")
    .entry("ini", "\"\"")
    .entry("java", "\"\"")
    .entry("jl", "\"\"")
    .entry("jpeg", "\"\"")
    .entry("jpg", "\"\"")
    .entry("js", "\"\"")
    .entry("json", "\"\"")
    .entry("jsx", "\"\"")
    .entry("ksh", "\"\"")
    .entry("less", "\"\"")
    .entry("lhs", "\"\"")
    .entry("lua", "\"\"")
    .entry("markdown", "\"\"")
    .entry("md", "\"\"")
    .entry("ml", "\"λ\"")
    .entry("mli", "\"λ\"")
    .entry("mustache", "\"\"")
    .entry("php", "\"\"")
    .entry("pl", "\"\"")
    .entry("pm", "\"\"")
    .entry("png", "\"\"")
    .entry("pp", "\"\"")
    .entry("ps1", "\"\"")
    .entry("psb", "\"\"")
    .entry("psd", "\"\"")
    .entry("py", "\"\"")
    .entry("pyc", "\"\"")
    .entry("pyd", "\"\"")
    .entry("pyo", "\"\"")
    .entry("rb", "\"\"")
    .entry("rlib", "\"\"")
    .entry("rmd", "\"\"")
    .entry("rs", "\"\"")
    .entry("rss", "\"\"")
    .entry("sass", "\"\"")
    .entry("scala", "\"\"")
    .entry("scss", "\"\"")
    .entry("sh", "\"\"")
    .entry("slim", "\"\"")
    .entry("sln", "\"\"")
    .entry("sql", "\"\"")
    .entry("styl", "\"\"")
    .entry("suo", "\"\"")
    .entry("t", "\"\"")
    .entry("ts", "\"\"")
    .entry("tsx", "\"\"")
    .entry("twig", "\"\"")
    .entry("vim", "\"\"")
    .entry("vue", "\"﵂\"")
    .entry("xul", "\"\"")
    .entry("yaml", "\"\"")
    .entry("yml", "\"\"")
    .entry("zsh", "\"\"")
    .build(&mut file)
    .unwrap();
  write!(&mut file, ";\n").unwrap();
}
