use std::path::PathBuf;
use std::fs;
use std::collections::{HashMap, HashSet};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use swc_common::SourceMap;
use swc_common::sync::Lrc;

use crate::lyy_config_visit::ConfigVisit;
use crate::lyy_parse::parse_ast;

#[derive(Default)]
pub struct Config {
    config_path: PathBuf,
    cm: Lrc<SourceMap>,
}

impl Config {
    pub fn new(path: &str) -> Self {
        Config {
            config_path: PathBuf::from(path),
            cm: Lrc::new(Default::default())
        }
    }

    pub fn edit_to_webpack(&self) {
        let mut ast = parse_ast(&self.config_path, &self.cm);

        let mut config_visitor = ConfigVisit {
            get_define: false
        };

        ast.visit_mut_with(&mut config_visitor);
    }
}