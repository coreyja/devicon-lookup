#[derive(Debug, Deserialize)]
pub struct Args {
    pub flag_color: bool,
    pub flag_iconcolor: bool,
    pub flag_long: bool,
    pub flag_nameshort: bool,
    pub flag_dirshort: bool,
    pub flag_dirshortreverse: bool,
    pub flag_version: bool,
    pub flag_fzf: bool,
    pub flag_regex: Option<String>,
    pub flag_prefix: Option<String>,
    pub flag_substitute: bool,
}
