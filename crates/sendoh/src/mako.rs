use std::path::PathBuf;
use std::collections::HashMap;

pub struct Mako {
    dist: PathBuf,
    module_ids: Vec<String>,
    code_map: HashMap<String, String>
}