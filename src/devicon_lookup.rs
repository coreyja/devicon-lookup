use std::io::{self, Write};
use ansi_term::Style;

pub use crate::file::File;
pub use crate::file_ext::FileExtensions;
pub use crate::devicon_lookup::icon::{FileIcon, Icons};
pub use crate::devicon_lookup::color::FileColours;

pub mod icon;
pub mod color;
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
            file: File::new(&filename)
        })
    }
}


impl ParsedLine {
    fn symbol(&self) -> char {
        let file_icon = Box::new(FileExtensions);

        let icon = icon::find_exact_name(&self.file.name);

        if icon.is_none() && self.file.is_dir {
            return *icon::find_direcotry(&self.file.name)
                    .unwrap_or(&Icons::Dir.value());
        } if let Some(icon) = file_icon.custom_match(&self.file) { return icon; }
        else if icon.is_none() && self.file.ext.is_some() {
            return *icon::find_extension(&self.file.ext)
                    .unwrap_or(&Icons::File.value());
        }
        return *icon.unwrap_or(&Icons::File.value());
    }

    fn color(&self) -> Style {
        let file_color = Box::new(FileExtensions);
        let style: Option<Style>;
        if self.file.is_dir {
            style = file_color.color_dir(&self.file);
        }
        else {
            style = file_color.color_file(&self.file);
        }
        return color::iconify_style(style.unwrap_or_default());
    }

    pub fn print_with_symbol(&self, is_iconcolor: bool, is_nameshort: bool, is_dirshort: bool, is_dirshort_reversed: bool, is_fzf: bool) {
        // println!("{} {} {}\n", is_iconcolor, is_nameshort, is_dirshort);
        let mut icon = self.symbol().to_string();
        if is_iconcolor {
            icon = self.color().paint(icon).to_string();
        }

        let mut path = self.original.clone();
        if is_nameshort {
            path = color::main_color().paint(self.file.name.clone()).to_string();
        } else if is_dirshort || is_dirshort_reversed {
            let colored_dirs = color::detail_color().paint(self.file.short_path(is_dirshort_reversed));
            let colored_name = color::main_color().paint(format!("{:<22}", self.file.name.clone()));
            path = format!("{:<22}{}", colored_name, colored_dirs);
        }

        let s;
        if is_fzf {
            s = format!("{}!{} {}\n", self.original, icon, path);
        } else {
            s = format!("{} {}\n", icon, path);
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
