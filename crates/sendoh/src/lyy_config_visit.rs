use swc_ecma_ast::{
    KeyValueProp, PropName, Str, ModuleItem, Stmt, ExprStmt, Decl, Pat, CallExpr,
    Callee, Expr, ObjectLit
};
use swc_ecma_visit::{VisitMut, VisitMutWith};

pub struct ConfigVisit {
    pub get_define: bool
}

impl VisitMut for ConfigVisit {
    fn visit_mut_call_expr(&mut self, n: &mut CallExpr) {
        if self.get_define {
            return;
        }
        let callee = &n.callee;
        if let Callee::Expr(expr) = callee {
            match &**expr {
                Expr::Ident(ident) => {
                    if ident.sym.to_string() == "defineConfig".to_string() {
                        println!("fing defineConfig");
                        self.get_define = true;
                        
                        let mut args = n.args.first_mut().unwrap();

                        let expr = args.expr.as_mut();

                        if let Expr::Object(obj) = expr {
                            let mut props = &obj.props;
                            // props.push();
                        }

                        println!("args: {:#?}", args);
                    }
                },
                _ => {}
            }
        }
    }
}
