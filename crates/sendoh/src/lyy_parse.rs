use std::path::PathBuf;

use swc_common::SourceMap;
use swc_common::sync::Lrc;
use swc_ecma_parser::{
    lexer::Lexer,
    Parser,
    EsConfig,
    Syntax,
    StringInput
};
use swc_ecma_codegen::{
    text_writer,
    Emitter
};
use swc_ecma_ast::{ EsVersion, Module, ModuleItem };

pub fn parse_ast(path: &PathBuf, cm: &Lrc<SourceMap>) -> Module {
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

pub fn transform_item(cm: &Lrc<SourceMap>, item: ModuleItem) -> String {
    // 声明 buf
    let mut buf = vec![];
    let mut map_buf = vec![];
    {
        // 根据 wr 创建 Emitter
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config::default()
                .with_minify(false)
                .with_target(EsVersion::Es5)
                .with_omit_last_semi(true),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(text_writer::JsWriter::new(
                cm.clone(), 
                "\n",
                &mut buf,
                Some(&mut map_buf),
            )),
        };

        emitter.emit_module_item(&item).expect("failed to emit module");
    }

    let code = String::from_utf8(buf.clone()).unwrap();

    code
}

pub fn transform_expr(cm: &Lrc<SourceMap>, ast: Module) -> String {
    // 声明 buf
    let mut buf = vec![];
    let mut map_buf = vec![];
    {
        // 根据 wr 创建 Emitter
        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config::default()
                .with_minify(false)
                .with_target(EsVersion::Es5)
                .with_omit_last_semi(true),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(text_writer::JsWriter::new(
                cm.clone(), 
                "\n",
                &mut buf,
                Some(&mut map_buf),
            )),
        };

        emitter.emit_module(&ast).expect("failed to emit module");
    }

    let code = String::from_utf8(buf.clone()).unwrap();

    code
}