pub mod encoder;
pub mod minifier;

pub use encoder::Encoder;
pub use minifier::minify;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_to_jsfuck(input: &str, enable_minify: bool) -> Result<String, JsValue> {
    let code = if enable_minify {
        // SWCによるMinifyを実行。失敗した場合はエラーをJS側に投げる。
        minify(input).map_err(|e| JsValue::from_str(&format!("Minification failed: {}", e)))?
    } else {
        input.to_string()
    };

    let encoded_string = Encoder::encode_string(&code);
    Ok(Encoder::wrap_execution(&encoded_string))
}
