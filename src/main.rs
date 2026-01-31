use std::env;
use std::fs;
use std::process;
use jsfuck_encoder::{Encoder, minify};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file.js>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let code = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    // Step 1: Minify
    let minified = match minify(&code) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Minification error: {}", e);
            process::exit(1);
        }
    };
    
    // セミコロン除去
    let minified_clean = minified.trim().trim_end_matches(';');
    eprintln!("Minified code: {}", minified_clean);

    // Step 2: Encode to string expression
    let encoded_string = Encoder::encode_string(minified_clean);

    // Step 3: Wrap in Function constructor for execution
    let final_code = Encoder::wrap_execution(&encoded_string);
    
    // 標準出力にはコードのみを出す（リダイレクト用）
    println!("{}", final_code);
    eprintln!("Final size: {} bytes", final_code.len());
}