use regex::Regex;
use regex::RegexSet;

#[derive(Debug)]
pub struct File {
    pub full_path: String,
    pub path: String,
    pub ext: Option<String>,
    pub is_dir: bool,
    pub name: String,
}

pub const DOTS: &str = "…";

impl File {
    pub fn new(full_path: &String) -> File {
        lazy_static! {
            static ref RE_PATH: Regex = Regex::new(
                format!(
                    "(?<path>.*?)(?<name>[^{0}]+){0}?$",
                    std::path::MAIN_SEPARATOR_STR
                )
                .as_str()
            )
            .unwrap();
        }

        let is_dir = full_path.ends_with(std::path::MAIN_SEPARATOR_STR);

        let re = RE_PATH.captures(full_path).unwrap();

        let name = re["name"].to_string();
        let path = re["path"].to_string();
        let ext = name.rfind('.').map(|p| name[p + 1..].to_owned());

        File {
            full_path: full_path.clone(),
            path,
            name,
            ext,
            is_dir,
        }
    }

    pub fn short_path(&self, is_reversed: bool) -> String {
        let mut c = 0;
        let mut new_path: Vec<String> = Vec::new();

        let arr: Vec<String> = self
            .path
            .split(std::path::MAIN_SEPARATOR_STR)
            .filter(|&x| !x.is_empty())
            .map(str::to_string)
            .collect();

        if arr.len() > 0 {
            new_path.push("".to_string());
        }

        let iter: Box<dyn Iterator<Item = _>> = if is_reversed {
            Box::new(arr.iter().rev().enumerate())
        } else {
            Box::new(arr.iter().enumerate())
        };

        for (e, is_last_element) in iter.map(|(i, e)| (e, i == arr.len() - 1)) {
            let is_first_dir: bool = if new_path.len() == 1 && is_reversed {
                true
            } else if is_last_element {
                true
            } else {
                false
            };

            if new_path.len() < 3 {
                new_path.push(File::short_path_part(e, is_first_dir));
            } else if new_path.len() == 3 && (arr.len() - c > 2) {
                new_path.push(DOTS.to_string());
            } else if arr.len() - c <= 2 && !e.is_empty() {
                new_path.push(File::short_path_part(e, is_first_dir));
            }

            c += 1;
        }

        let join_symbol;
        if is_reversed {
            join_symbol = "<";
        } else {
            join_symbol = std::path::MAIN_SEPARATOR_STR;
        }

        new_path
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(join_symbol)
    }

    pub fn short_path_part(e: &String, is_ext_size: bool) -> String {
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
            r = e.clone()
        }
        r
    }

    pub fn extension_is_one_of(&self, choices: &[&str]) -> bool {
        match &self.ext {
            Some(ext) => choices.contains(&&ext[..]),
            None => false,
        }
    }
    pub fn extension_matches_set(&self, set: &RegexSet) -> bool {
        match &self.ext {
            Some(ext) => set.is_match(&&ext[..]),
            None => false,
        }
    }

    pub fn name_is_one_of(&self, choices: &[&str]) -> bool {
        choices.contains(&&self.name[..])
    }
    pub fn name_matches_set(&self, set: &RegexSet) -> bool {
        set.is_match(&&self.name[..])
    }
}
