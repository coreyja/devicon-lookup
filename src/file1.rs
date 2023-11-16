use std::path::Path;
use regex::RegexSet;

pub struct File {
    pub filename: String,
    pub path: Path,
    // pub ext: Option<String>,
}

impl File {
    pub fn from_filename(filename: String) -> File
    {
        let path = Path::new(filename);
        // let name = filename.clone().trim_end_matches('/');
        // let ext  = path.extension()?.to_str();

        // File { path, name, ext };
        File { filename, path }
    }
    /// A file’s name is derived from its string. This needs to handle directories
    /// such as `/` or `..`, which have no `file_name` component. So instead, just
    /// use the last component as the name.

    /// Whether this file is a directory on the filesystem.
    // pub fn is_directory(&self) -> bool {
    //     return self.path.is_dir()
    // }

    // /// Whether this file is a regular file on the filesystem — that is, not a
    // /// directory, a link, or anything else treated specially.
    // pub fn is_file(&self) -> bool {
    //     self.path.is_file()
    // }

    // /// Whether this file’s extension is any of the strings that get passed in.
    // ///
    // /// This will always return `false` if the file has no extension.
    // pub fn extension_is_one_of(&self, choices: &[&str]) -> bool {
    //     match &self.ext {
    //         Some(ext)  => choices.contains(&&ext[..]),
    //         None       => false,
    //     }
    // }
    // pub fn extension_matches_set(&self, set: RegexSet) -> bool {
    //     match &self.ext {
    //         Some(ext)  => set.is_match(&&ext[..]),
    //         None       => false,
    //     }
    // }

    // /// Whether this file’s name, including extension, is any of the strings
    // /// that get passed in.
    // pub fn name_is_one_of(&self, choices: &[&str]) -> bool {
    //     choices.contains(&&self.name[..])
    // }
    // pub fn name_matches_set(&self, set: RegexSet) -> bool {
    //     set.is_match(&&self.name[..])
    // }

}
