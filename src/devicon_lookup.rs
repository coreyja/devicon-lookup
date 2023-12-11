use ansi_term::Style;
use std::io::{self, Write};

use crate::args::Args;
pub use crate::devicon_lookup::color::FileColours;
pub use crate::devicon_lookup::icon::{FileIcon, Icons};
pub use crate::file::File;
pub use crate::file_ext::FileExtensions;

pub mod color;
pub mod icon;
pub mod parsers;

pub trait ParserFn: Fn(String) -> ParserResult {}

impl<T: Fn(String) -> ParserResult> ParserFn for T {}

pub type ParserResult = Result<String, &'static str>;
pub type Parser = dyn ParserFn;

pub struct ParsedLine {
    original: String,
    file: File,
}

pub struct Line<'a> {
    pub original: String,
    pub filename_parsers: Vec<&'a Parser>,
}

pub struct LineBuilder<'a> {
    original: String,
    filename_parsers: Vec<&'a Parser>,
}

impl<'a> LineBuilder<'a> {
    pub fn new(original: String) -> Self {
        Self {
            original,
            filename_parsers: vec![],
        }
    }

    pub fn with_parser(&mut self, parser: &'a Parser) {
        self.filename_parsers.push(parser);
    }

    pub fn build(self) -> Line<'a> {
        Line {
            original: self.original,
            filename_parsers: self.filename_parsers,
        }
    }
}

impl<'a> Line<'a> {
    pub fn parse(self) -> Result<ParsedLine, &'static str> {
        let mut curr: ParserResult = Ok(self.original.clone());

        for parser in self.filename_parsers.iter() {
            curr = parser(curr?)
        }

        let filename = curr?;
        Ok(ParsedLine {
            original: self.original,
            file: File::new(&filename),
        })
    }
}

impl ParsedLine {
    fn symbol(&self) -> char {
        let file_icon = Box::new(FileExtensions);

        let icon = icon::find_exact_name(&self.file.name());

        if icon.is_none() && self.file.is_dir() {
            return *icon::find_direcotry(&self.file.name()).unwrap_or(&Icons::Dir.value());
        }
        if let Some(icon) = file_icon.custom_match(&self.file) {
            return icon;
        } else if icon.is_none() && self.file.ext().is_some() {
            return *icon::find_extension(&self.file.ext()).unwrap_or(&Icons::File.value());
        }
        return *icon.unwrap_or(&Icons::File.value());
    }

    fn color(&self) -> Style {
        let file_color = Box::new(FileExtensions);
        let style: Option<Style>;
        if self.file.is_dir() {
            style = file_color.color_dir(&self.file);
        } else {
            style = file_color.color_file(&self.file);
        }
        return color::iconify_style(style.unwrap_or_default());
    }

    fn get_icon(&self, flag_iconcolor: bool) -> String {
        let mut icon = self.symbol().to_string();
        if flag_iconcolor {
            icon = self.color().paint(icon).to_string();
        }
        icon
    }

    fn get_name(&self, flag_long: bool, flag_nameshort: bool, flag_align: Option<usize>) -> String {
        let name = if flag_nameshort {
            File::short_path_part(&self.file.name(), true)
        } else {
            self.file.name().to_owned()
        };
        if flag_long {
            let out_name = match flag_align {
                Some(i) => {
                    let len = i - name.len() - 1;
                    if i > name.len() + 1 {
                        return name + &" ".repeat(len);
                    }
                    return name;
                }
                None => name,
            };
            // .paint(format!("{:<22}", name))
            color::main_color().paint(out_name).to_string()
        } else {
            name
        }
    }

    fn get_path(
        &self,
        flag_long: bool,
        flag_dirshort: bool,
        flag_dirshortreverse: bool,
        flag_nodir: bool,
    ) -> String {
        if flag_nodir {
            return "".to_string();
        }
        let path = if flag_dirshort || flag_dirshortreverse {
            self.file.short_path(flag_dirshortreverse)
        } else {
            self.file.path().to_owned()
        };

        if flag_long {
            color::detail_color().paint(path).to_string()
        } else {
            path
        }
    }

    pub fn print_with_symbol(&self, args: &Args) {
        let icon = self.get_icon(args.flag_iconcolor);

        let name = self.get_name(args.flag_long, args.flag_nameshort, args.flag_align);
        let path = self.get_path(
            args.flag_long,
            args.flag_dirshort,
            args.flag_dirshortreverse,
            args.flag_nodir,
        );

        let path_name = if args.flag_nodir {
            name
        } else if args.flag_long {
            format!("{} {}", name, path)
        } else {
            format!("{}{}", path, name)
        };

        let mut s = if args.flag_color {
            format!(
                "{} {}",
                icon,
                self.original.replace(&self.file.full_path(), &path_name)
            )
        } else {
            format!("{} {}", icon, path_name)
        };

        if args.flag_substitute || args.flag_prefix.is_some() {
            s = format!("{}", self.original.replace(&self.file.full_path(), &s));
        }

        if args.flag_fzf {
            s = format!("{}!{}", &self.file.full_path(), s);
        }
        if !s.ends_with("\n") {
            s += "\n";
        }
        write_to_stdout(s.as_bytes())
    }
}

pub fn write_to_stdout(item: &[u8]) {
    let write_result = io::stdout().write_all(item);

    if write_result.is_err() {
        ::std::process::exit(0)
    }
}
