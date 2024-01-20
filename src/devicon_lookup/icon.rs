use phf::phf_map;

extern crate lazy_static;

#[derive(Copy, Clone)]
pub enum Icons {
    Audio,
    Image,
    Video,
    File,
    Dir,
}

impl Icons {
    pub fn value(self) -> char {
        match self {
            Self::Audio => '\u{f001}',
            Self::Image => '\u{f1c5}',
            Self::Video => '\u{f03d}',
            Self::File => '\u{f15b}', // 
            Self::Dir => '\u{f07c}',  // 
        }
    }
}

static EXACT_NAME_MAP: phf::Map<&'static str, char> = phf_map! {
    ".Trash" => '\u{f1f8}',
    ".atom" => '\u{e764}',
    ".bashprofile" => '\u{e615}',
    ".bashrc" => '\u{ebc7}',
    ".git" => '\u{e702}',
    ".gitattributes" => '\u{e702}',
    ".gitconfig" => '\u{e702}',
    ".github" => '\u{f408}',
    ".gitignore" => '\u{e702}',
    ".gitmodules" => '\u{e702}',
    ".rvm" => '\u{e21e}',
    ".vimrc" => '\u{e62b}',
    ".vscode" => '\u{e70c}',
    ".zshrc" => '\u{ebc7}',
    "Cargo.lock" => '\u{e7a8}',
    "bin" => '\u{e5fc}',
    "config" => '\u{e5fc}',
    "docker-compose.yml" => '\u{f308}',
    "Dockerfile" => '\u{f308}',
    "ds_store" => '\u{f179}',
    "gitignore_global" => '\u{e702}',
    "go.mod" => '\u{e626}',
    "go.sum" => '\u{e626}',
    "gradle" => '\u{e256}',
    "gruntfile.coffee" => '\u{e611}',
    "gruntfile.js" => '\u{e611}',
    "gruntfile.ls" => '\u{e611}',
    "gulpfile.coffee" => '\u{e610}',
    "gulpfile.js" => '\u{e610}',
    "gulpfile.ls" => '\u{e610}',
    "hidden" => '\u{f023}',
    "include" => '\u{e5fc}',
    "lib" => '\u{f121}',
    "localized" => '\u{f179}',
    "Makefile" => '\u{ebc7}',
    "node_modules" => '\u{e718}',
    "npmignore" => '\u{e71e}',
    "PKGBUILD" => '\u{f303}',
    "rubydoc" => '\u{e73b}',
    "yarn.lock" => '\u{e718}',
    "README.md" => '\u{ea61}',
};

static DIRECTORY_MAP: phf::Map<&'static str, char> = phf_map! {
    "bin"           => '\u{e5fc}', // 
    ".git"          => '\u{e702}', // 
    ".idea"         => '\u{e7b5}', // 
    ".vscode"       => '\u{e70c}', //
};

static EXTENSION_MAP: phf::Map<&'static str, char> = phf_map! {
    "ai"            => '\u{e7b4}', // 
    "android"       => '\u{e70e}', // 
    "apk"           => '\u{e70e}', // 
    "apple"         => '\u{f179}', // 
    "avi"           => '\u{f03d}', // 
    "avif"          => '\u{f1c5}', // 
    "avro"          => '\u{e60b}', // 
    "awk"           => '\u{ebc7}', // 
    "bash"          => '\u{ebc7}', // 
    "bash_history"  => '\u{ebc7}', // 
    "bash_profile"  => '\u{ebc7}', // 
    "bashrc"        => '\u{ebc7}', // 
    "bat"           => '\u{ebc4}', // 
    "bats"          => '\u{ebc7}', // 
    "bmp"           => '\u{f1c5}', // 
    "bz"            => '\u{f410}', // 
    "bz2"           => '\u{f410}', // 
    "c"             => '\u{e61e}', // 
    "c++"           => '\u{e61d}', // 
    "cab"           => '\u{e70f}', // 
    "cc"            => '\u{e61d}', // 
    "cfg"           => '\u{e615}', // 
    "class"         => '\u{e256}', // 
    "clj"           => '\u{e768}', // 
    "cljs"          => '\u{e76a}', // 
    "cls"           => '\u{f034}', // 
    "cmd"           => '\u{e70f}', // 
    "coffee"        => '\u{f0f4}', // 
    "conf"          => '\u{e615}', // 
    "cp"            => '\u{e61d}', // 
    "cpio"          => '\u{f410}', // 
    "cpp"           => '\u{e61d}', // 
    "cs"            => '\u{e648}', // 
    "csh"           => '\u{ebc7}', // 
    "cshtml"        => '\u{f1fa}', // 
    "csproj"        => '\u{f0a1e}', // 
    "css"           => '\u{e749}', // 
    "csv"           => '\u{f1c3}', // 
    "csx"           => '\u{f81a}', // 
    "cxx"           => '\u{e61d}', // 
    "d"             => '\u{e7af}', // 
    "dart"          => '\u{e798}', // 
    "db"            => '\u{f1c0}', // 
    "deb"           => '\u{e77d}', // 
    "diff"          => '\u{f440}', // 
    "djvu"          => '\u{f02d}', // 
    "dll"           => '\u{e70f}', // 
    "doc"           => '\u{f1c2}', // 
    "docx"          => '\u{f1c2}', // 
    "ds_store"      => '\u{f179}', // 
    "DS_store"      => '\u{f179}', // 
    "dump"          => '\u{f1c0}', // 
    "ebook"         => '\u{e28b}', // 
    "ebuild"        => '\u{f30d}', // 
    "editorconfig"  => '\u{e615}', // 
    "ejs"           => '\u{e618}', // 
    "elm"           => '\u{e62c}', // 
    "env"           => '\u{f462}', // 
    "eot"           => '\u{f031}', // 
    "epub"          => '\u{e28a}', // 
    "erb"           => '\u{e73b}', // 
    "erl"           => '\u{e7b1}', // 
    "ex"            => '\u{e62d}', // 
    "exe"           => '\u{f17a}', // 
    "exs"           => '\u{e62d}', // 
    "fish"          => '\u{ebc7}', // 
    "flac"          => '\u{f001}', // 
    "flv"           => '\u{f03d}', // 
    "font"          => '\u{f031}', // 
    "fs"            => '\u{e7a7}', // 
    "fsi"           => '\u{e7a7}', // 
    "fsx"           => '\u{e7a7}', // 
    "gdoc"          => '\u{f1c2}', // 
    "gem"           => '\u{e21e}', // 
    "gemfile"       => '\u{e21e}', // 
    "gemspec"       => '\u{e21e}', // 
    "gform"         => '\u{f298}', // 
    "gif"           => '\u{f1c5}', // 
    "git"           => '\u{e702}', // 
    "gitattributes" => '\u{e702}', // 
    "gitignore"     => '\u{e702}', // 
    "gitmodules"    => '\u{e702}', // 
    "go"            => '\u{e626}', // 
    "gradle"        => '\u{e256}', // 
    "groovy"        => '\u{e775}', // 
    "gsheet"        => '\u{f1c3}', // 
    "gslides"       => '\u{f1c4}', // 
    "guardfile"     => '\u{e21e}', // 
    "gz"            => '\u{f410}', // 
    "h"             => '\u{f0fd}', // 
    "hbs"           => '\u{e60f}', // 
    "hpp"           => '\u{f0fd}', // 
    "hs"            => '\u{e777}', // 
    "htm"           => '\u{f13b}', // 
    "html"          => '\u{f13b}', // 
    "hxx"           => '\u{f0fd}', // 
    "ico"           => '\u{f1c5}', // 
    "image"         => '\u{f1c5}', // 
    "img"           => '\u{e271}', // 
    "iml"           => '\u{e7b5}', // 
    "ini"           => '\u{e615}', // 
    "ipynb"         => '\u{e606}', // 
    "iso"           => '\u{e271}', // 
    "j2c"           => '\u{f1c5}', // 
    "j2k"           => '\u{f1c5}', // 
    "jad"           => '\u{e256}', // 
    "jar"           => '\u{e256}', // 
    "java"          => '\u{e256}', // 
    "jfi"           => '\u{f1c5}', // 
    "jfif"          => '\u{f1c5}', // 
    "jif"           => '\u{f1c5}', // 
    "jl"            => '\u{e624}', // 
    "jmd"           => '\u{f48a}', // 
    "jp2"           => '\u{f1c5}', // 
    "jpe"           => '\u{f1c5}', // 
    "jpeg"          => '\u{f1c5}', // 
    "jpg"           => '\u{f1c5}', // 
    "jpx"           => '\u{f1c5}', // 
    "js"            => '\u{e74e}', // 
    "json"          => '\u{e60b}', // 
    "jsx"           => '\u{e7ba}', // 
    "jxl"           => '\u{f1c5}', // 
    "ksh"           => '\u{ebc7}', // 
    "latex"         => '\u{f034}', // 
    "less"          => '\u{e758}', // 
    "lhs"           => '\u{e777}', // 
    "license"       => '\u{f718}', // 
    "localized"     => '\u{f179}', // 
    "lock"          => '\u{f023}', // 
    "log"           => '\u{f18d}', // 
    "lua"           => '\u{e620}', // 
    "lz"            => '\u{f410}', // 
    "lz4"           => '\u{f410}', // 
    "lzh"           => '\u{f410}', // 
    "lzma"          => '\u{f410}', // 
    "lzo"           => '\u{f410}', // 
    "m"             => '\u{e61e}', // 
    "mm"            => '\u{e61d}', // 
    "m4a"           => '\u{f001}', // 
    "markdown"      => '\u{f48a}', // 
    "md"            => '\u{f48a}', // 
    "mjs"           => '\u{e74e}', // 
    "mk"            => '\u{ebc7}', // 
    "mkd"           => '\u{f48a}', // 
    "mkv"           => '\u{f03d}', // 
    "mobi"          => '\u{e28b}', // 
    "mov"           => '\u{f03d}', // 
    "mp3"           => '\u{f001}', // 
    "mp4"           => '\u{f03d}', // 
    "msi"           => '\u{e70f}', // 
    "mustache"      => '\u{e60f}', // 
    "nix"           => '\u{f313}', // 
    "node"          => '\u{f898}', // 
    "npmignore"     => '\u{e71e}', // 
    "odp"           => '\u{f1c4}', // 
    "ods"           => '\u{f1c3}', // 
    "odt"           => '\u{f1c2}', // 
    "ogg"           => '\u{f001}', // 
    "ogv"           => '\u{f03d}', // 
    "otf"           => '\u{f031}', // 
    "part"          => '\u{f43a}', // 
    "patch"         => '\u{f440}', // 
    "pdf"           => '\u{f1c1}', // 
    "php"           => '\u{e73d}', // 
    "pl"            => '\u{e67e}', // 
    "plx"           => '\u{e769}', // 
    "pm"            => '\u{e769}', // 
    "png"           => '\u{f1c5}', // 
    "pod"           => '\u{e769}', // 
    "ppt"           => '\u{f1c4}', // 
    "pptx"          => '\u{f1c4}', // 
    "procfile"      => '\u{e21e}', // 
    "properties"    => '\u{e60b}', // 
    "ps1"           => '\u{ebc7}', // 
    "psd"           => '\u{e7b8}', // 
    "pxm"           => '\u{f1c5}', // 
    "py"            => '\u{e606}', // 
    "pyc"           => '\u{e606}', // 
    "r"             => '\u{f25d}', // 
    "rakefile"      => '\u{e21e}', // 
    "rar"           => '\u{f410}', // 
    "razor"         => '\u{f1fa}', // 
    "rb"            => '\u{e21e}', // 
    "rdata"         => '\u{f25d}', // 
    "rdb"           => '\u{e76d}', // 
    "rdoc"          => '\u{f48a}', // 
    "rds"           => '\u{f25d}', // 
    "readme"        => '\u{ea61}', // 
    "rlib"          => '\u{e7a8}', // 
    "rmd"           => '\u{f48a}', // 
    "rpm"           => '\u{e7bb}', // 
    "rs"            => '\u{e7a8}', // 
    "rspec"         => '\u{e21e}', // 
    "rspec_parallel"=> '\u{e21e}', // 
    "rspec_status"  => '\u{e21e}', // 
    "rss"           => '\u{f09e}', // 
    "rtf"           => '\u{f718}', // 
    "ru"            => '\u{e21e}', // 
    "rubydoc"       => '\u{e73b}', // 
    "sass"          => '\u{e603}', // 
    "scala"         => '\u{e737}', // 
    "scss"          => '\u{e749}', // 
    "sh"            => '\u{ebc7}', // 
    "shell"         => '\u{ebc7}', // 
    "slim"          => '\u{e73b}', // 
    "sln"           => '\u{e70c}', // 
    "so"            => '\u{f17c}', // 
    "sql"           => '\u{f1c0}', // 
    "styl"          => '\u{e600}', // 
    "stylus"        => '\u{e600}', // 
    "svg"           => '\u{f1c5}', // 
    "swift"         => '\u{e755}', // 
    "t"             => '\u{e67e}', // 
    "tar"           => '\u{f410}', // 
    "taz"           => '\u{f410}', // 
    "tbz"           => '\u{f410}', // 
    "tbz2"          => '\u{f410}', // 
    "tex"           => '\u{f034}', // 
    "tgz"           => '\u{f410}', // 
    "tiff"          => '\u{f1c5}', // 
    "tlz"           => '\u{f410}', // 
    "toml"          => '\u{e615}', // 
    "torrent"       => '\u{e275}', // 
    "ts"            => '\u{e628}', // 
    "tsv"           => '\u{f1c3}', // 
    "tsx"           => '\u{e7ba}', // 
    "ttf"           => '\u{f031}', // 
    "twig"          => '\u{e61c}', // 
    "txt"           => '\u{f15c}', // 
    "txz"           => '\u{f410}', // 
    "tz"            => '\u{f410}', // 
    "tzo"           => '\u{f410}', // 
    "video"         => '\u{f03d}', // 
    "vim"           => '\u{e62b}', // 
    "vue"           => '\u{e6a0}', // ﵂
    "war"           => '\u{e256}', // 
    "wav"           => '\u{f001}', // 
    "webm"          => '\u{f03d}', // 
    "webp"          => '\u{f1c5}', // 
    "windows"       => '\u{f17a}', // 
    "woff"          => '\u{f031}', // 
    "woff2"         => '\u{f031}', // 
    "xhtml"         => '\u{f13b}', // 
    "xls"           => '\u{f1c3}', // 
    "xlsx"          => '\u{f1c3}', // 
    "xml"           => '\u{f121}', // 
    "xul"           => '\u{f121}', // 
    "xz"            => '\u{f410}', // 
    "yaml"          => '\u{f481}', // 
    "yml"           => '\u{f481}', // 
    "zip"           => '\u{f410}', // 
    "zsh"           => '\u{ebc7}', // 
    "zsh-theme"     => '\u{ebc7}', // 
    "zshrc"         => '\u{ebc7}', // 
    "zst"           => '\u{f410}', // 
    "xi"            => '\u{f168}', // 
    "chm"           => '\u{ebcc}', // 
    "7z"            => '\u{f410}', // 
};

pub fn find_exact_name(filename: &str) -> Option<char> {
    // I made this method the ones below return `Option<char>` instead of `Option<&char>`
    // I did this since `char` is Copy in Rust so its 'easy' to make a new one by dereferencing.
    // The only usage of this dereferenced the return value right away so I thought it made since
    // for this to just return a `char` directly
    EXACT_NAME_MAP.get(filename).copied()
}

pub fn find_directory(filename: &str) -> Option<char> {
    DIRECTORY_MAP.get(filename).copied()
}

pub fn find_extension(extension: &str) -> Option<char> {
    EXTENSION_MAP.get(extension).copied()
}
