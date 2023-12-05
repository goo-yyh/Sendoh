use std::collections::HashMap;

use swc_ecma_ast::{
    KeyValueProp, PropName, Str, ModuleItem, Stmt, ExprStmt, Decl, Pat
};
use swc_ecma_visit::{Visit, VisitWith};
use swc_common::{DUMMY_SP, SourceMap};
use swc_common::sync::Lrc;

use crate::lyy_parse::transform_item;

pub struct WebpackAsyncVisit {
    cm: Lrc<SourceMap>,
    pub module_ids: Vec<String>,
    pub module_map: HashMap<String, String>
}

impl WebpackAsyncVisit {
    pub fn new(cm: Lrc<SourceMap>) -> Self {
        WebpackAsyncVisit {
            cm,
            module_ids: Vec::new(),
            module_map: HashMap::new()
        }
    }
}

impl Visit for WebpackAsyncVisit {
    fn visit_key_value_prop(&mut self, n: &KeyValueProp) {
        if let KeyValueProp {
            key: PropName::Str(Str {
                value: val,
                ..
            }),
            value,
        } = n {
            self.module_ids.push(val.to_string());

            let item = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: value.clone()
            }));

            let code = transform_item(&self.cm, item);

            self.module_map.insert(val.to_string(), code);
        }
    }
}

pub struct WebpackMainVisit {
    cm: Lrc<SourceMap>,
    find: bool,
    pub module_ids: Vec<String>,
    pub module_map: HashMap<String, String>
}

impl WebpackMainVisit {
    pub fn new(cm: Lrc<SourceMap>) -> Self {
        WebpackMainVisit {
            cm,
            find: false,
            module_ids: Vec::new(),
            module_map: HashMap::new()
        }
    }
}

impl Visit for WebpackMainVisit {
    fn visit_decl(&mut self,n: &Decl) {
        if self.find {
            return;
        }
        if let Decl::Var(var) = n {
            if let Some(decl) = var.decls.first() {
                if let Pat::Ident(ident) = &decl.name {
                    if ident.id.sym == "__webpack_modules__" {
                        self.find = true;

                        n.visit_children_with(self);
                    }
                }
            }
        }
    }

    fn visit_key_value_prop(&mut self, n: &KeyValueProp) {
        if !self.find {
            return;
        }
        if let KeyValueProp {
            key: PropName::Str(Str {
                value: val,
                ..
            }),
            value,
        } = n {
            self.module_ids.push(val.to_string());

            let item = ModuleItem::Stmt(Stmt::Expr(ExprStmt {
                span: DUMMY_SP,
                expr: value.clone()
            }));

            let code = transform_item(&self.cm, item);

            self.module_map.insert(val.to_string(), code);
        }
    }
}