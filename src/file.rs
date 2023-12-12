use itertools::Itertools;
use regex::RegexSet;
use std::path::PathBuf;

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
        let parent = self.path.parent().unwrap();
        let component_count = parent.components().count();

        let iter: Box<dyn Iterator<Item = _>> = if is_reversed {
            Box::new(parent.components().rev())
        } else {
            Box::new(parent.components())
        };

        let join_symbol = if is_reversed {
            "<"
        } else {
            std::path::MAIN_SEPARATOR_STR
        };

        let short_path = iter
            .enumerate()
            .map(|(i, component)| -> String {
                if i < 3 || component_count - i <= 2 {
                    let is_innermost_dir = if is_reversed {
                        i == 0
                    } else {
                        i == component_count - 1
                    };

                    File::short_path_part(
                        &component.as_os_str().to_string_lossy(),
                        is_innermost_dir,
                    )
                } else {
                    DOTS.to_string()
                }
            })
            .join(join_symbol);

        format!("{short_path}{join_symbol}")
    }

    pub fn short_path_part(e: &str, is_ext_size: bool) -> String {
        // I removed the `let r` bit since we can just return the result of the `if` directly
        // In Rust you can use the return value of `if` as a value, which is nice for returning things
        // conditionally filling in a variable

        let max_len = if is_ext_size { 20 } else { 10 };

        if e.len() > max_len {
            let a: String = e.chars().take(max_len / 2).collect();
            let b: String = e
                .chars()
                .skip(e.len() - max_len / 2)
                .take(max_len)
                .collect();

            a + DOTS + &b
        } else {
            e.to_owned()
        }
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

    pub(crate) fn path(&self) -> String {
        // PathBug::parent returns everything but the last component of the path
        let parent = self.path.parent().unwrap().to_str().unwrap();

        // This is to pass the existing tests
        // If the parent is empty we want to leave it off,
        // otherwise we want to add a `/` to the end
        if parent.is_empty() {
            parent.to_string()
        } else {
            format!("{}/", parent)
        }
    }

    pub(crate) fn full_path(&self) -> &str {
        &self.original
    }
}
