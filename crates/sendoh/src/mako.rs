use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};

use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_visit::VisitWith;

use crate::lyy_parse::parse_ast;
use crate::lyy_mako_visit::{MakoAsyncVisit, MakoMainVisit};

#[derive(Default)]
pub struct Mako {
    dist: PathBuf,
    cm: Lrc<SourceMap>,
    pub module_ids: HashSet<String>,
    pub code_map: HashMap<String, String>
}

impl Mako {
    pub fn new(path: &str) -> Self {
        Mako {
            dist: PathBuf::from(path),
            module_ids: HashSet::new(),
            code_map: HashMap::new(),
            cm: Lrc::new(Default::default())
        }
    }

    pub fn get_files(&mut self) {
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

    pub fn parse_file(&mut self, path: &PathBuf, is_main: bool) {
        let ast = parse_ast(path, &self.cm);
        
        if !is_main {
            let mut async_visitor = MakoAsyncVisit::new(self.cm.clone());
            ast.visit_with(&mut async_visitor);
            let ids = async_visitor.module_ids;
            let map = async_visitor.module_map;

            for id in ids {
                self.module_ids.insert(id.clone());
                self.code_map.insert(id.clone(), map.get(&id).unwrap().to_string());
            }
        } else {
            let mut main_visitor = MakoMainVisit::new(self.cm.clone());
            ast.visit_with(&mut main_visitor);

            let ids = main_visitor.module_ids;
            let map = main_visitor.module_map;

            for id in ids {
                self.module_ids.insert(id.clone());
                self.code_map.insert(id.clone(), map.get(&id).unwrap().to_string());
            }
        }
        
    }
}