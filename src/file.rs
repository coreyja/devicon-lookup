use regex::RegexSet;

pub struct File {
    pub path: String,
    pub path_arr: Vec<String>,
    pub ext: Option<String>,
    pub is_dir: bool,
    pub name: String,
}

pub const DOTS: &str = "…";

impl File {
    pub fn new(filename: &String) -> File {
        let mut path_arr: Vec<String> = filename
            .rsplit(std::path::MAIN_SEPARATOR_STR)
            .filter(|&x| !x.is_empty())
            .map(str::to_string).collect();
        let name = path_arr[0].clone();
        path_arr.remove(0);

        File {
            path: filename.clone(), 
            path_arr,
            name,
            ext: File::ext(filename),
            is_dir: File::is_dir(filename),
        }
    }

    fn ext(filename: &String) -> Option<String> {
        filename.rfind('.')
            .map(|p| filename[p + 1 ..].to_owned())
    }       
    
    fn is_dir(filename: &String) -> bool {
        filename.ends_with(std::path::MAIN_SEPARATOR_STR)
    } 

    pub fn short_path(&self) -> String {
        let mut c = 0;
        let mut new_path: Vec<String> = Vec::new();

        let arr = &self.path_arr;

        if arr.len() > 0 {
            new_path.push("".to_string());
        }
        for e in arr.iter() {
            if new_path.len() < 2 {
                new_path.push(File::short_path_part(e));
            } else if new_path.len() == 2 && (arr.len() - c > 2) {
                new_path.push(DOTS.to_string());
            } else if arr.len() - c <= 2 && !e.is_empty() {
                new_path.push(File::short_path_part(e));
            }
            
            c += 1;
        }
        new_path.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("<")
    }

    fn short_path_part(e: &String) -> String {
        let r: String;
        if e.len() > 8 {
            let a: String = e.chars().take(4).collect();
            let b: String = e.chars().skip(e.len() - 4).take(4).collect();
            r = a + DOTS + &b;
        } else {
            r = e.clone()
        }
        r
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
