use std::path::PathBuf;

use regex::RegexSet;

#[derive(Debug)]
pub struct File {
    original: String,
    path: PathBuf,
}

pub const DOTS: &str = "…";

impl File {
    pub fn new(full_path: &str) -> File {
        let original = full_path.to_string();

        File {
            original,
            path: PathBuf::from(full_path),
        }
    }

    pub fn short_path(&self, is_reversed: bool) -> String {
        todo!()
        // let mut c = 0;
        // let mut new_path: Vec<String> = Vec::new();

        // let arr: Vec<String> = self
        //     .path
        //     .split(std::path::MAIN_SEPARATOR_STR)
        //     .filter(|&x| !x.is_empty())
        //     .map(str::to_string)
        //     .collect();

        // if arr.len() > 0 {
        //     new_path.push("".to_string());
        // }

        // let iter: Box<dyn Iterator<Item = _>> = if is_reversed {
        //     Box::new(arr.iter().rev().enumerate())
        // } else {
        //     Box::new(arr.iter().enumerate())
        // };

        // for (e, is_last_element) in iter.map(|(i, e)| (e, i == arr.len() - 1)) {
        //     let is_first_dir: bool = if new_path.len() == 1 && is_reversed {
        //         true
        //     } else if is_last_element {
        //         true
        //     } else {
        //         false
        //     };

        //     if new_path.len() < 3 {
        //         new_path.push(File::short_path_part(e, is_first_dir));
        //     } else if new_path.len() == 3 && (arr.len() - c > 2) {
        //         new_path.push(DOTS.to_string());
        //     } else if arr.len() - c <= 2 && !e.is_empty() {
        //         new_path.push(File::short_path_part(e, is_first_dir));
        //     }

        //     c += 1;
        // }

        // let join_symbol;
        // if is_reversed {
        //     join_symbol = "<";
        // } else {
        //     join_symbol = std::path::MAIN_SEPARATOR_STR;
        // }

        // new_path
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<_>>()
        //     .join(join_symbol)
    }

    pub fn short_path_part(e: &str, is_ext_size: bool) -> String {
        let r: String;
        let max_len = if is_ext_size { 20 } else { 10 };

        if e.len() > max_len {
            let a: String = e.chars().take(max_len / 2).collect();
            let b: String = e
                .chars()
                .skip(e.len() - max_len / 2)
                .take(max_len)
                .collect();
            r = a + DOTS + &b;
        } else {
            r = e.to_owned()
        }
        r
    }

    pub fn extension_is_one_of(&self, choices: &[&str]) -> bool {
        match &self.ext() {
            Some(ext) => choices.contains(&&ext[..]),
            None => false,
        }
    }
    pub fn extension_matches_set(&self, set: &RegexSet) -> bool {
        match &self.ext() {
            Some(ext) => set.is_match(ext),
            None => false,
        }
    }

    pub fn name_is_one_of(&self, choices: &[&str]) -> bool {
        choices.contains(&self.name())
    }
    pub fn name_matches_set(&self, set: &RegexSet) -> bool {
        set.is_match(self.name())
    }

    pub(crate) fn name(&self) -> &str {
        // `file_name()` here _might_ not return something if the String is only `..`
        // but we are choosing to ignore that error here and unwrap. We will panic on `..` input
        // I believe this would have also panicked on the previous impl because the regex would't have matched

        // The second unwrap is because `PathBuf` works on `OsString` which is not guaranteed to be valid unicode
        // This library only works on valid unicode, so we unwrap here to panic if it's not valid unicode
        self.path.file_name().unwrap().to_str().unwrap()
    }

    pub(crate) fn is_dir(&self) -> bool {
        // I thought I wanted to use `PathBuf::is_dir` here, but that actually checks the FileSystem
        // and your version was avoiding the FS (which I like!)
        // At the moment I added in the original path so we can check if it ended with a `/`
        // May refactor this later!
        self.original.ends_with(std::path::MAIN_SEPARATOR)
    }

    pub(crate) fn ext(&self) -> Option<&str> {
        // `PathBuf::extension` is what we want but returns an `OsString` again
        // This `map` would work great! But I wanted to show off `?` to return `None` early
        // self.path.extension().map(|x| x.to_str().unwrap())

        // Here `ext` is an `OsString`! If `extension()` returns `None` then this function
        // will early return `None`. I like this style of `map` often since it looks nicer to me personally
        let ext = self.path.extension()?;

        ext.to_str()
    }

    pub(crate) fn path(&self) -> &str {
        // PathBug::parent returns everything but the last component of the path
        self.path.parent().unwrap().to_str().unwrap()
    }

    pub(crate) fn full_path(&self) -> &str {
        &self.original
    }
}
