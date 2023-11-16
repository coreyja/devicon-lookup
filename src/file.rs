use std::path::Path;
use regex::RegexSet;

pub struct File {
    pub fullname: String,
    pub name: String,
    pub ext: Option<String>,
    pub is_dir: bool,
}

impl File {
    pub fn from_filename(filename: String) -> File
    {
        let name = filename.trim_end_matches('/').to_string();
        
        let path = Path::new(&filename);
        let ext = path.extension()
            .and_then(|x| x.to_str())
            .map(|x| "".to_owned() + x);
        let is_dir = path.is_dir();
        File {
            fullname: filename, 
            name,
            ext,
            is_dir
        }
    }

    pub fn extension_is_one_of(&self, choices: &[&str]) -> bool {
        match &self.ext {
            Some(ext)  => choices.contains(&&ext[..]),
            None       => false,
        }
    }
    pub fn extension_matches_set(&self, set: &RegexSet) -> bool {
        match &self.ext {
            Some(ext)  => set.is_match(&&ext[..]),
            None       => false,
        }
    }

    pub fn name_is_one_of(&self, choices: &[&str]) -> bool {
        choices.contains(&&self.name[..])
    }
    pub fn name_matches_set(&self, set: &RegexSet) -> bool {
        set.is_match(&&self.name[..])
    }    

}
