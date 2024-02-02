use itertools::Itertools;
use regex::RegexSet;
use std::path::{Component, PathBuf};

#[derive(Debug)]
pub struct File {
    original: String,
    path: PathBuf,
    is_dir: bool,
}

pub const DOTS: &str = "…";

impl File {
    pub fn new(full_path: &str) -> File {
        let original = full_path.to_string();
        let path = PathBuf::from(full_path);

        let is_dir = full_path.ends_with(std::path::MAIN_SEPARATOR) || path.is_dir();

        File {
            original,
            is_dir,
            path,
        }
    }

    pub fn short_path(&self, is_reversed: bool) -> String {
        let parent = self.path.parent();

        let Some(parent) = parent else {
            return "/".to_string();
        };

        let filtered_parent = parent
            .components()
            .filter(|&x| x != Component::RootDir)
            .collect::<PathBuf>();

        let component_count = filtered_parent.components().count();

        let join_symbol = if is_reversed {
            "<"
        } else {
            std::path::MAIN_SEPARATOR_STR
        };

        let iter: Box<dyn Iterator<Item = _>> = if is_reversed {
            Box::new(filtered_parent.components().rev())
        } else {
            Box::new(filtered_parent.components())
        };

        let mut is_dots_added = false;
        let save_n_first_dir = 2;
        let save_n_last_dir = 2;

        let short_path = iter
            .enumerate()
            .filter_map(|(i, component)| -> Option<String> {
                if i < save_n_first_dir || component_count - i <= save_n_last_dir {
                    let is_innermost_dir = if is_reversed {
                        i == 0
                    } else {
                        i == component_count - 1
                    };

                    Some(File::short_path_part(
                        &component.as_os_str().to_string_lossy(),
                        is_innermost_dir,
                    ))
                } else if !is_dots_added {
                    is_dots_added = true;
                    Some(DOTS.to_string())
                } else {
                    None
                }
            })
            .join(join_symbol);

        if component_count > 0 && (is_reversed || filtered_parent != parent) {
            format!("{join_symbol}{short_path}")
        } else {
            short_path
        }
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
            Some(ext) => choices.contains(ext),
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
        let Some(name) = self.name() else {
            return false;
        };
        choices.contains(&name)
    }

    pub fn name_matches_set(&self, set: &RegexSet) -> bool {
        let Some(name) = self.name() else {
            return false;
        };
        set.is_match(name)
    }

    pub(crate) fn name(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }

    pub(crate) fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub(crate) fn ext(&self) -> Option<&str> {
        self.path.extension()?.to_str()
    }

    pub(crate) fn path(&self) -> String {
        // PathBug::parent returns everything but the last component of the path
        let parent = self
            .path
            .parent()
            .map(|os| {
                os.to_str()
                    .expect("All paths should be valid UTF-8 since we divert non-UTF-8 beforehand")
            })
            .map(String::from);

        // If the parent is empty we want to leave it off,
        // otherwise we want to add a `/` to the end
        if let Some(parent) = parent {
            if parent.is_empty() {
                parent.to_string()
            } else {
                format!("{}/", parent)
            }
        } else {
            "".to_string()
        }
    }

    pub(crate) fn full_path(&self) -> &str {
        &self.original
    }
}
