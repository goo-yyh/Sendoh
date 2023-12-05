use std::path::PathBuf;
use std::sync::Arc;

use swc_common::SourceMap;
use swc_ecma_parser::{
    lexer::Lexer,
    Parser,
    EsConfig,
    Syntax,
    StringInput
};
use swc_ecma_ast::{ EsVersion, Module };

pub fn parse_ast(path: &PathBuf, cm: &Arc<SourceMap>) -> Module {
    let fm = cm.load_file(path.as_path())
        .expect("failed to read program file");

    let syntax = Syntax::Es(EsConfig {
        decorators: true,
        decorators_before_export: true,
        ..Default::default()
    });

    let lexer = Lexer::new(
        syntax,
        EsVersion::Es2015,
        StringInput::from(&*fm),
        None
    );

    let mut parser = Parser::new_from(lexer);

    // parse to ast
    let ast = parser.parse_module().unwrap();

    ast
}