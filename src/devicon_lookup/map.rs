use phf::phf_map;

static SYMBOL_MAP: phf::Map<&'static str, &'static str> = phf_map! {
        "ai" => "",
        "awk" => "",
        "bash" => "",
        "bat" => "",
        "bmp" => "",
        "c" => "",
        "c++" => "",
        "cc" => "",
        "clj" => "",
        "cljc" => "",
        "cljs" => "",
        "coffee" => "",
        "conf" => "",
        "cp" => "",
        "cpp" => "",
        "cs" => "",
        "csh" => "",
        "css" => "",
        "cxx" => "",
        "d" => "",
        "dart" => "",
        "db" => "",
        "diff" => "",
        "dump" => "",
        "edn" => "",
        "eex" => "",
        "ejs" => "",
        "elm" => "",
        "erl" => "",
        "ex" => "",
        "exs" => "",
        "f#" => "",
        "fish" => "",
        "fs" => "",
        "fsi" => "",
        "fsscript" => "",
        "fsx" => "",
        "gemspec" => "",
        "gif" => "",
        "go" => "",
        "h" => "",
        "haml" => "",
        "hbs" => "",
        "heex" => "",
        "hh" => "",
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
        "leex" => "",
        "less" => "",
        "lhs" => "",
        "lua" => "",
        "markdown" => "",
        "md" => "",
        "mdx" => "",
        "mjs" => "",
        "mk" => "",
        "ml" => "λ",
        "mli" => "λ",
        "mustache" => "",
        "nix" => "",
        "pem" => "",
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
        "r" => "ﳒ",
        "rake" => "",
        "rb" => "",
        "rlib" => "",
        "rmd" => "",
        "rproj" => "鉶",
        "rs" => "",
        "rss" => "",
        "sass" => "",
        "scala" => "",
        "scss" => "",
        "sh" => "",
        "slim" => "",
        "sln" => "",
        "sol" => "ﲹ",
        "sql" => "",
        "styl" => "",
        "suo" => "",
        "swift" => "",
        "t" => "",
        "tex" => "ﭨ",
        "toml" => "",
        "ts" => "",
        "tsx" => "",
        "twig" => "",
        "vim" => "",
        "vue" => "﵂",
        "webmanifest" => "",
        "webp" => "",
        "xcplayground" => "",
        "xul" => "",
        "yaml" => "",
        "yml" => "",
        "zsh" => "",
};

pub fn find_symbol(extenstion: &str) -> &str {
    SYMBOL_MAP.get(extenstion).unwrap_or(&crate::DEFAULT_SYMBOL)
}
