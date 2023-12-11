use crate::file::File;
use regex::RegexSet;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileExtensions;

extern crate lazy_static;

impl FileExtensions {
    /// An “immediate” file is something that can be run or activated somehow
    /// in order to kick off the build of a project. It’s usually only present
    /// in directories full of source code.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_immediate(&self, file: &File) -> bool {
        file.name().ends_with(".ninja")
            || file.name_is_one_of(&[
                "Makefile",
                "Cargo.toml",
                "SConstruct",
                "CMakeLists.txt",
                "build.gradle",
                "pom.xml",
                "Rakefile",
                "package.json",
                "Gruntfile.js",
                "Gruntfile.coffee",
                "BUILD",
                "BUILD.bazel",
                "WORKSPACE",
                "build.xml",
                "Podfile",
                "webpack.config.js",
                "meson.build",
                "composer.json",
                "RoboFile.php",
                "PKGBUILD",
                "Justfile",
                "Procfile",
                "Dockerfile",
                "Containerfile",
                "Vagrantfile",
                "Brewfile",
                "Gemfile",
                "Pipfile",
                "build.sbt",
                "mix.exs",
                "bsconfig.json",
                "tsconfig.json",
            ])
    }

    pub fn is_image(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "png", "jfi", "jfif", "jif", "jpe", "jpeg", "jpg", "gif", "bmp", "tiff", "tif", "ppm",
            "pgm", "pbm", "pnm", "webp", "raw", "arw", "svg", "stl", "eps", "dvi", "ps", "cbr",
            "jpf", "cbz", "xpm", "ico", "cr2", "orf", "nef", "heif", "avif", "jxl", "j2k", "jp2",
            "j2c", "jpx",
        ])
    }

    pub fn is_video(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "avi", "flv", "m2v", "m4v", "mkv", "mov", "mp4", "mpeg", "mpg", "ogm", "ogv", "vob",
            "wmv", "webm", "m2ts", "heic",
        ])
    }

    pub fn is_music(&self, file: &File) -> bool {
        file.extension_is_one_of(&["aac", "m4a", "mp3", "ogg", "wma", "mka", "opus"])
    }

    // Lossless music, rather than any other kind of data...
    pub fn is_lossless(&self, file: &File) -> bool {
        file.extension_is_one_of(&["alac", "ape", "flac", "wav"])
    }

    pub fn is_crypto(&self, file: &File) -> bool {
        file.extension_is_one_of(&["asc", "enc", "gpg", "pgp", "sig", "signature", "pfx", "p12"])
    }

    pub fn is_document(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "djvu",
            "doc",
            "docx",
            "dvi",
            "eml",
            "eps",
            "fotd",
            "key",
            "keynote",
            "numbers",
            "odp",
            "odt",
            "pages",
            "pdf",
            "ppt",
            "pptx",
            "rtf",
            "xls",
            "xlsx",
            "txt",
            "md",
            "log",
            "changelog",
        ]) || file.name_is_one_of(&["readme", "LICENSE", "LICENCE"])
    }

    pub fn is_config(&self, file: &File) -> bool {
        lazy_static! {
            static ref SET_EXT: RegexSet = RegexSet::new(&[
                r".*ignore",
                r".*settings.*",
                r".*theme.*",
                r".*settings.*",
                r".*theme.*",
            ])
            .unwrap();
            static ref SET_NAME: RegexSet =
                RegexSet::new(&[r".*settings.*", r".*theme.*", r".*settings.*", r".*theme.*",])
                    .unwrap();
        }
        file.extension_is_one_of(&[
            "toml",
            "ini",
            "conf",
            "config",
            "yml",
            "gitignore",
            "gitmodules",
        ]) || file.name_is_one_of(&["conf", "config"])
            || file.extension_matches_set(&SET_EXT)
            || file.name_matches_set(&SET_NAME)
    }

    pub fn is_compressed(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "zip", "tar", "Z", "z", "gz", "bz2", "a", "ar", "7z", "iso", "dmg", "tc", "rar", "par",
            "tgz", "xz", "txz", "lz", "tlz", "lzma", "deb", "rpm", "zst", "lz4", "cpio",
        ])
    }

    pub fn is_temp(&self, file: &File) -> bool {
        file.name().ends_with('~')
            || (file.name().starts_with('#') && file.name().ends_with('#'))
            || file.extension_is_one_of(&["tmp", "swp", "swo", "swn", "bak", "bkp", "bk"])
            || file.extension_is_one_of(&["tmp", "swp", "swo", "swn", "bak", "bkp", "bk"])
    }

    pub fn is_compiled(&self, file: &File) -> bool {
        if file.extension_is_one_of(&["class", "elc", "hi", "o", "pyc", "zwc", "ko"]) {
            true
        } else {
            false
        }
    }

    pub fn is_script(&self, file: &File) -> bool {
        lazy_static! {
            static ref SET_SH_EXT: RegexSet =
                RegexSet::new(&[r".*bash.*", r".*zsh.*", r"^sh_", r"_sh$",]).unwrap();
        }
        file.extension_is_one_of(&[
            "bashrc",
            "zshrc",
            "sh",
            "zsh",
            "bash",
            "profile",
            "bash_profile",
            "fish",
        ]) || file.extension_matches_set(&SET_SH_EXT)
    }

    pub fn is_language(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "as",
            "ai",
            "applescript",
            "scpt",
            "cs",
            "csproj",
            "cpp",
            "hpp",
            "c",
            "h",
            "clj",
            "coffee",
            "css",
            "erb",
            "less",
            "sass",
            "erb",
            "liquid",
            "lua",
            "scss",
            "styl",
            "d",
            "ex",
            "erl",
            "eot",
            "otf",
            "ttf",
            "woff",
            "go",
            "dot",
            "gv",
            "js",
            "hbs",
            "hs",
            "jade",
            "java",
            "jsp",
            "jl",
            "lisp",
            "matlab",
            "js",
            "mustache",
            "m",
            "mm",
            "ml",
            "pl",
            "pm",
            "t",
            "php",
            "psd",
            "pug",
            "py",
            "pp",
            "r",
            "rails",
            "rb",
            "rs",
            "scala",
            "gradle",
            "swift",
            "tcl",
            "tex",
            "textile",
            "vb",
        ])
    }

    pub fn is_pretty_data(&self, file: &File) -> bool {
        file.extension_is_one_of(&[
            "json", "xml", "hx", "hxml", "ctp", "haml", "html", "slim", "sql",
        ])
    }

    pub fn is_vim(&self, file: &File) -> bool {
        lazy_static! {
            static ref NAME_EXT: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
            static ref SET_EXT: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
        }
        file.name_matches_set(&NAME_EXT) || file.extension_matches_set(&SET_EXT)
    }

    pub fn is_config_folder(&self, file: &File) -> bool {
        file.name_is_one_of(&["docker"])
    }

    pub fn is_folder_with_language_files(&self, file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r".*test.*",]).unwrap();
        }
        file.name_is_one_of(&["src", "lib"]) || file.name_matches_set(&SET_NAME)
    }

    pub fn is_folder_with_exe_files(&self, file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r".*script.*",]).unwrap();
        }
        file.name_is_one_of(&["target", "bin"]) || file.name_matches_set(&SET_NAME)
    }

    pub fn is_folder_with_document_files(&self, file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r"man.*",]).unwrap();
        }
        file.name_is_one_of(&["doc"]) || file.name_matches_set(&SET_NAME)
    }
}
