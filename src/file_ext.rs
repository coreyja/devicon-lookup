use crate::{devicon_lookup::icon::Icons, file::File};
use ansi_term::{Colour::*, Style};
use regex::RegexSet;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileExtensions;

extern crate lazy_static;

impl FileExtensions {
    pub(crate) fn custom_match(file: &File) -> Option<char> {
        lazy_static! {
            static ref SET_VIM_NAME: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
            static ref SET_VIM_EXT: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
            static ref SET_SH_EXT: RegexSet =
                RegexSet::new(&[r".*bash.*", r".*zsh.*", r"^sh_", r"_sh$",]).unwrap();
        }

        if FileExtensions::is_music(file) || FileExtensions::is_lossless(file) {
            Some(Icons::Audio.value())
        } else if FileExtensions::is_image(file) {
            Some(Icons::Image.value())
        } else if FileExtensions::is_video(file) {
            Some(Icons::Video.value())
        } else if file.extension_matches_set(&SET_VIM_EXT) || file.name_matches_set(&SET_VIM_NAME) {
            Some('\u{e62b}')
        } else if file.extension_matches_set(&SET_SH_EXT) {
            Some('\u{ebc7}')
        } else {
            None
        }
    }

    pub(crate) fn color_file(file: &File) -> Option<Style> {
        Some(match file {
            f if FileExtensions::is_temp(f) => Fixed(244).normal(),
            f if FileExtensions::is_immediate(f) => Fixed(1).bold().underline(),
            f if FileExtensions::is_image(f) => Fixed(37).normal(),
            f if FileExtensions::is_video(f) => Fixed(135).normal(),
            f if FileExtensions::is_music(f) => Fixed(92).normal(),
            f if FileExtensions::is_lossless(f) => Fixed(93).normal(),
            f if FileExtensions::is_crypto(f) => Fixed(109).normal(),
            f if FileExtensions::is_document(f) => Fixed(187).normal(),
            f if FileExtensions::is_compressed(f) => Red.normal(),
            f if FileExtensions::is_compiled(f) => Fixed(137).normal(),
            f if FileExtensions::is_pretty_data(f) => Fixed(178).normal(),
            f if FileExtensions::is_script(f) => Fixed(173).normal(),
            f if FileExtensions::is_config(f) => Fixed(65).normal(),
            f if FileExtensions::is_vim(f) => Fixed(71).normal(),
            f if FileExtensions::is_language(f) => Fixed(75).normal(),
            _ => Fixed(244).bold(),
        })
    }

    pub(crate) fn color_dir(file: &File) -> Option<Style> {
        Some(match file {
            f if FileExtensions::is_config_folder(f) => Fixed(65).bold(),
            f if FileExtensions::is_folder_with_language_files(f) => Fixed(75).bold(),
            f if FileExtensions::is_folder_with_exe_files(f) => Fixed(173).bold(),
            f if FileExtensions::is_folder_with_document_files(f) => Fixed(187).bold(),
            _ => Fixed(244).bold(),
        })
    }

    /// An “immediate” file is something that can be run or activated somehow
    /// in order to kick off the build of a project. It’s usually only present
    /// in directories full of source code.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    pub fn is_immediate(file: &File) -> bool {
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

    pub fn is_image(file: &File) -> bool {
        file.extension_is_one_of(&[
            "png", "jfi", "jfif", "jif", "jpe", "jpeg", "jpg", "gif", "bmp", "tiff", "tif", "ppm",
            "pgm", "pbm", "pnm", "webp", "raw", "arw", "svg", "stl", "eps", "dvi", "ps", "cbr",
            "jpf", "cbz", "xpm", "ico", "cr2", "orf", "nef", "heif", "avif", "jxl", "j2k", "jp2",
            "j2c", "jpx",
        ])
    }

    pub fn is_video(file: &File) -> bool {
        file.extension_is_one_of(&[
            "avi", "flv", "m2v", "m4v", "mkv", "mov", "mp4", "mpeg", "mpg", "ogm", "ogv", "vob",
            "wmv", "webm", "m2ts", "heic",
        ])
    }

    pub fn is_music(file: &File) -> bool {
        file.extension_is_one_of(&["aac", "m4a", "mp3", "ogg", "wma", "mka", "opus"])
    }

    // Lossless music, rather than any other kind of data...
    pub fn is_lossless(file: &File) -> bool {
        file.extension_is_one_of(&["alac", "ape", "flac", "wav"])
    }

    pub fn is_crypto(file: &File) -> bool {
        file.extension_is_one_of(&["asc", "enc", "gpg", "pgp", "sig", "signature", "pfx", "p12"])
    }

    pub fn is_document(file: &File) -> bool {
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

    pub fn is_config(file: &File) -> bool {
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

    pub fn is_compressed(file: &File) -> bool {
        file.extension_is_one_of(&[
            "zip", "tar", "Z", "z", "gz", "bz2", "a", "ar", "7z", "iso", "dmg", "tc", "rar", "par",
            "tgz", "xz", "txz", "lz", "tlz", "lzma", "deb", "rpm", "zst", "lz4", "cpio",
        ])
    }

    pub fn is_temp(file: &File) -> bool {
        file.name().ends_with('~')
            || (file.name().starts_with('#') && file.name().ends_with('#'))
            || file.extension_is_one_of(&["tmp", "swp", "swo", "swn", "bak", "bkp", "bk"])
            || file.extension_is_one_of(&["tmp", "swp", "swo", "swn", "bak", "bkp", "bk"])
    }

    pub fn is_compiled(file: &File) -> bool {
        if file.extension_is_one_of(&["class", "elc", "hi", "o", "pyc", "zwc", "ko"]) {
            true
        } else {
            false
        }
    }

    pub fn is_script(file: &File) -> bool {
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

    pub fn is_language(file: &File) -> bool {
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

    pub fn is_pretty_data(file: &File) -> bool {
        file.extension_is_one_of(&[
            "json", "xml", "hx", "hxml", "ctp", "haml", "html", "slim", "sql",
        ])
    }

    pub fn is_vim(file: &File) -> bool {
        lazy_static! {
            static ref NAME_EXT: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
            static ref SET_EXT: RegexSet = RegexSet::new(&[r".*vim.*",]).unwrap();
        }
        file.name_matches_set(&NAME_EXT) || file.extension_matches_set(&SET_EXT)
    }

    pub fn is_config_folder(file: &File) -> bool {
        file.name_is_one_of(&["docker"])
    }

    pub fn is_folder_with_language_files(file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r".*test.*",]).unwrap();
        }
        file.name_is_one_of(&["src", "lib"]) || file.name_matches_set(&SET_NAME)
    }

    pub fn is_folder_with_exe_files(file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r".*script.*",]).unwrap();
        }
        file.name_is_one_of(&["target", "bin"]) || file.name_matches_set(&SET_NAME)
    }

    pub fn is_folder_with_document_files(file: &File) -> bool {
        lazy_static! {
            static ref SET_NAME: RegexSet = RegexSet::new(&[r"man.*",]).unwrap();
        }
        file.name_is_one_of(&["doc"]) || file.name_matches_set(&SET_NAME)
    }
}
