use swc_core::common::{sync::Lrc, FileName, SourceMap};
use swc_core::ecma::parser::{lexer::Lexer, Parser, StringInput, Syntax};
use swc_core::ecma::codegen::{text_writer::JsWriter, Emitter, Config};

pub fn minify(code: &str) -> Result<String, String> {
    let cm: Lrc<SourceMap> = Default::default();
    
    // 修正: code.to_string() で所有権のある文字列を渡す
    let fm = cm.new_source_file(FileName::Anon.into(), code.to_string());

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);
    
    // 修正: エラーメッセージの詳細出力を省略（波括弧回避のため）
    let module = parser
        .parse_module()
        .map_err(|_| "Parse error".to_string())?;

    let mut buf = vec![];
    {
        let mut cfg = Config::default();
        cfg.minify = true;

        let mut emitter = Emitter {
            cfg,
            cm: cm.clone(),
            comments: None,
            wr: JsWriter::new(cm, "\n", &mut buf, None),
        };
        
        // 修正: エラーメッセージの詳細出力を省略
        emitter.emit_module(&module)
            .map_err(|_| "Emit error".to_string())?;
    }

    // 修正: エラーメッセージの詳細出力を省略
    String::from_utf8(buf).map_err(|_| "UTF-8 error".to_string())
}
