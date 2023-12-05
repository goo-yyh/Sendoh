use std::path::PathBuf;
use std::fs;
use std::sync::Arc;
use std::collections::HashMap;

use swc_common::SourceMap;

use crate::lyy_parse::parse_ast;
use crate::lyy_visit::WebpackAsyncVisit;

#[derive(Default)]
pub struct Webpack {
    dist: PathBuf,
    cm: Arc<SourceMap>,
    module_ids: Vec<String>,
    code_map: HashMap<String, String>
}

impl Webpack {
    pub fn new(path: &str) -> Self {
        Webpack {
            dist: PathBuf::from(path),
            module_ids: Vec::new(),
            code_map: HashMap::new(),
            cm: Arc::new(Default::default())
        }
    }

    pub fn get_files(&self) {
        let dir = fs::read_dir(&self.dist).unwrap();
        for file in dir {
            let entry = file.unwrap();
            let name = entry.file_name();
            let file_name = name.to_str().unwrap();
            let is_async = file_name.ends_with("async.js");
            let is_main = !is_async && file_name.ends_with(".js");

            if file_name.ends_with(".js") {
                println!("name: {}", file_name);
                self.parse_file(&self.dist.join(file_name), is_main);
            }
        }
    }

    pub fn parse_file(&self, path: &PathBuf, is_main: bool) {
        let file_path = path.to_str().unwrap();
        let ast = parse_ast(path, &self.cm);

        println!("file_path: {}", file_path);
    }
}