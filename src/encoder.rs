pub struct Encoder;

// キーワード生成戦略を表すEnum
enum GenStrategy {
    Simple(&'static str),                 // 固定文字列
    Func(fn() -> String),                 // 関数呼び出し
    Chars(&'static str),                  // 文字列から生成
}

impl Encoder {
    pub fn encode_string(input: &str) -> String {
        if input.is_empty() { return "[]+[]".to_string(); }
        
        let mut parts = vec!["[]".to_string()];
        let chars: Vec<char> = input.chars().collect();
        let mut i = 0;
        
        // 予約語のマッピングテーブル
        let keywords: Vec<(&str, GenStrategy)> = vec![
            ("constructor", GenStrategy::Func(Self::get_constructor_string)),
            ("instanceof", GenStrategy::Chars("instanceof")),
            ("undefined",  GenStrategy::Simple("([][[]]+[])")),
            ("arguments",  GenStrategy::Chars("arguments")),
            ("function",   GenStrategy::Func(Self::get_word_function)),
            ("toString",   GenStrategy::Func(Self::get_tostring_string)),
            ("Infinity",   GenStrategy::Func(Self::get_word_infinity)),
            ("continue",   GenStrategy::Chars("continue")),
            ("debugger",   GenStrategy::Chars("debugger")),
            
            ("default",    GenStrategy::Chars("default")),
            ("extends",    GenStrategy::Chars("extends")),
            ("finally",    GenStrategy::Chars("finally")),
            
            ("return",     GenStrategy::Func(Self::get_word_return)),
            ("switch",     GenStrategy::Chars("switch")),
            ("typeof",     GenStrategy::Chars("typeof")),
            ("delete",     GenStrategy::Chars("delete")),
            ("import",     GenStrategy::Chars("import")),
            ("export",     GenStrategy::Chars("export")),
            ("object",     GenStrategy::Chars("object")),
            
            ("false",      GenStrategy::Simple("(![]+[])")),
            ("const",      GenStrategy::Chars("const")),
            ("class",      GenStrategy::Chars("class")),
            ("break",      GenStrategy::Chars("break")),
            ("catch",      GenStrategy::Chars("catch")),
            ("throw",      GenStrategy::Chars("throw")),
            ("while",      GenStrategy::Chars("while")),
            ("super",      GenStrategy::Chars("super")),
            ("yield",      GenStrategy::Chars("yield")),
            ("async",      GenStrategy::Chars("async")),
            ("await",      GenStrategy::Chars("await")),
            
            ("true",       GenStrategy::Simple("(!![]+[])")),
            ("null",       GenStrategy::Chars("null")),
            ("this",       GenStrategy::Chars("this")),
            ("else",       GenStrategy::Chars("else")),
            ("case",       GenStrategy::Chars("case")),
            ("void",       GenStrategy::Chars("void")),
            ("with",       GenStrategy::Chars("with")),
            ("enum",       GenStrategy::Chars("enum")),
            ("from",       GenStrategy::Chars("from")),
            
            ("NaN",        GenStrategy::Simple("(+[![]]+[])")),
            ("new",        GenStrategy::Chars("new")),
            ("var",        GenStrategy::Chars("var")),
            ("let",        GenStrategy::Chars("let")),
            ("for",        GenStrategy::Chars("for")),
            ("try",        GenStrategy::Chars("try")),
            
            ("if",         GenStrategy::Chars("if")),
            ("in",         GenStrategy::Chars("in")),
            ("do",         GenStrategy::Chars("do")),
            ("of",         GenStrategy::Chars("of")),
        ];

        while i < chars.len() {
            let remaining: String = chars[i..].iter().collect();
            let mut matched = false;

            for (key, strategy) in &keywords {
                if remaining.starts_with(key) {
                    let code = match strategy {
                        GenStrategy::Simple(s) => s.to_string(),
                        GenStrategy::Func(f) => f(),
                        GenStrategy::Chars(s) => Self::word_from_chars(s),
                    };
                    parts.push(code);
                    i += key.len();
                    matched = true;
                    break;
                }
            }

            if !matched {
                parts.push(Self::encode_char(chars[i]));
                i += 1;
            }
        }
        
        parts.join("+")
    }

    pub fn wrap_execution(jsfuck_string_expr: &str) -> String {
        let func = Self::get_function_constructor();
        format!("{}({})()", func, jsfuck_string_expr)
    }

    // --- Optimized Word Helpers ---

    fn get_word_function() -> String { Self::word_from_chars("function") }
    fn get_word_return() -> String { Self::word_from_chars("return") }
    
    fn get_word_infinity() -> String {
        let one = "!![]";
        let e = Self::char_from_true(3); // e
        let zero = "+[]";
        let num_str = format!("({} +{} +{} +{} +{} +{})", one, e, one, zero, zero, zero);
        format!("(+{}+[])", num_str)
    }
    
    fn word_from_chars(word: &str) -> String {
        let mut parts = vec!["[]".to_string()];
        for c in word.chars() {
            parts.push(Self::encode_char(c));
        }
        format!("({})", parts.join("+"))
    }

    // --- Core Strings ---

    fn get_function_constructor() -> String {
        let str_filter = Self::get_filter_string();
        let str_constructor = Self::get_constructor_string();
        format!("[][{}] [{}]", str_filter, str_constructor)
    }

    fn get_constructor_string() -> String {
        let c = Self::char_from_object(5);
        let o = Self::char_from_object(1);
        let n = Self::char_from_undefined(1);
        let s = Self::char_from_false(3);
        let t = Self::char_from_true(0);
        let r = Self::char_from_true(1);
        let u = Self::char_from_undefined(0);
        format!("([]+{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{})", 
            c, o, n, s, t, r, u, c, t, o, r)
    }

    fn get_filter_string() -> String {
        let f = Self::char_from_false(0);
        let i = Self::char_from_undefined(5);
        let l = Self::char_from_false(2);
        let t = Self::char_from_true(0);
        let e = Self::char_from_true(3);
        let r = Self::char_from_true(1);
        format!("([]+{} +{} +{} +{} +{} +{})", f, i, l, t, e, r)
    }

    fn get_tostring_string() -> String {
        let t = Self::char_from_true(0);
        let o = Self::char_from_object(1);
        let big_s = Self::char_from_string_constructor(9);
        let r = Self::char_from_true(1);
        let i = Self::char_from_undefined(5);
        let n = Self::char_from_undefined(1);
        let g = Self::char_from_string_constructor(14);
        format!("([]+{} +{} +{} +{} +{} +{} +{} +{})", t, o, big_s, t, r, i, n, g)
    }

    // --- Helpers ---

    fn encode_index(n: u32) -> String {
        if n == 1 { return "+!![]".to_string(); }
        Self::encode_number(n)
    }

    fn encode_number(n: u32) -> String {
        if n == 0 { return "+[]".to_string(); }
        let mut parts = Vec::new();
        for _ in 0..n { parts.push("!![]"); }
        let s = parts.join("+");
        if n > 1 { format!("({})", s) } else { s }
    }

    // --- Character Source Mapping ---

    fn char_from_false(idx: u32) -> String { format!("(![]+[])[{}]", Self::encode_index(idx)) }
    fn char_from_true(idx: u32) -> String { format!("(!![]+[])[{}]", Self::encode_index(idx)) }
    fn char_from_undefined(idx: u32) -> String { format!("([][[]]+[])[{}]", Self::encode_index(idx)) }
    fn char_from_nan(idx: u32) -> String { format!("(+[![]]+[])[{}]", Self::encode_index(idx)) }
    fn char_from_object(idx: u32) -> String { format!("([]+{{}})[{}]", Self::encode_index(idx)) }
    
    fn char_from_string_constructor(idx: u32) -> String {
        let constr = Self::get_constructor_string();
        format!("(([]+[])[{}]+[])[{}]", constr, Self::encode_index(idx))
    }
    
    fn char_from_number_constructor(idx: u32) -> String {
        let constr = Self::get_constructor_string();
        format!("((+[])[{}]+[])[{}]", constr, Self::encode_index(idx))
    }

    fn char_from_array_constructor(idx: u32) -> String {
        let constr = Self::get_constructor_string();
        format!("([][{}]+[])[{}]", constr, Self::encode_index(idx))
    }

    fn char_from_boolean_constructor(idx: u32) -> String {
        let constr = Self::get_constructor_string();
        format!("((!![])[{}]+[])[{}]", constr, Self::encode_index(idx))
    }

    fn char_from_radix36(val: u32) -> String {
        let to_string_str = Self::get_tostring_string();
        let num_expr = if val == 1 { "+!![]".to_string() } else { Self::encode_number(val) };
        let radix = Self::encode_number(36);
        format!("(({})[{}]({})[+[]])", num_expr, to_string_str, radix) 
    }

    // --- Main Encoder ---

    fn encode_char(c: char) -> String {
        match c {
            'a' => Self::char_from_false(1),
            'b' => Self::char_from_object(2),
            'c' => Self::char_from_object(5),
            'd' => Self::char_from_undefined(2),
            'e' => Self::char_from_true(3),
            'f' => Self::char_from_false(0),
            'g' => Self::char_from_string_constructor(14),
            'h' => Self::char_from_radix36(17),
            'i' => Self::char_from_undefined(5),
            'j' => Self::char_from_object(3),
            'k' => Self::char_from_radix36(20),
            'l' => Self::char_from_false(2),
            'm' => Self::char_from_number_constructor(11),
            'n' => Self::char_from_undefined(1),
            'o' => Self::char_from_object(1),
            'p' => Self::char_from_radix36(25),
            'q' => Self::char_from_radix36(26),
            'r' => Self::char_from_true(1),
            's' => Self::char_from_false(3),
            't' => Self::char_from_true(0),
            'u' => Self::char_from_undefined(0),
            'v' => Self::char_from_radix36(31),
            'w' => Self::char_from_radix36(32),
            'x' => Self::char_from_radix36(33),
            'y' => Self::char_from_array_constructor(13),
            'z' => Self::char_from_radix36(35),

            'A' => Self::char_from_array_constructor(9),
            'B' => Self::char_from_boolean_constructor(9),
            'F' => format!("({}+[])[{}]", Self::get_function_constructor(), Self::encode_index(9)),
            'N' => Self::char_from_nan(0),
            'O' => Self::char_from_object(8),
            'S' => Self::char_from_string_constructor(9),

            '0'..='9' => {
                let d = c.to_digit(10).unwrap();
                format!("({}+[])", Self::encode_index(d))
            },

            ' ' => Self::char_from_object(7),
            '[' => Self::char_from_object(0),
            ']' => Self::char_from_object(14),
            '(' => format!("({}+[])[{}]", Self::get_function_constructor(), Self::encode_index(17)),
            ')' => format!("({}+[])[{}]", Self::get_function_constructor(), Self::encode_index(18)),
            '{' => format!("({}+[])[{}]", Self::get_function_constructor(), Self::encode_index(20)),
            
            '.' => {
                let italics = format!("([]+[]+{} +{} +{} +{} +{} +{} +{})", 
                    Self::char_from_undefined(5), // i
                    Self::char_from_true(0),      // t
                    Self::char_from_false(1),     // a
                    Self::char_from_false(2),     // l
                    Self::char_from_undefined(5), // i
                    Self::char_from_object(5),    // c
                    Self::char_from_false(3)      // s
                );
                let get_slash = format!("(([]+[])[{}]()+[])[{}]", italics, Self::encode_index(4));
                let code_str = format!("([]+{} +{} +{} +{} +{} +{} +{} +{} +{} +{})",
                    Self::char_from_true(1), // r
                    Self::char_from_true(3), // e
                    Self::char_from_true(0), // t
                    Self::char_from_undefined(0), // u
                    Self::char_from_true(1), // r
                    Self::char_from_undefined(1), // n
                    Self::char_from_object(7), // space
                    Self::encode_number(1),
                    get_slash,
                    Self::encode_number(2)
                );
                // 修正: (Function(...)() + [])[1] として、数値の0.5を文字列化してからインデックスアクセスする
                format!("({}({})()+[])[{}]", Self::get_function_constructor(), code_str, Self::encode_index(1))
            },

            _ => Self::char_from_unescape(c),
        }
    }
    
    fn char_from_unescape(c: char) -> String {
        let unescape_code = format!("([]+{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{})",
            Self::char_from_true(1), // r
            Self::char_from_true(3), // e
            Self::char_from_true(0), // t
            Self::char_from_undefined(0), // u
            Self::char_from_true(1), // r
            Self::char_from_undefined(1), // n
            Self::char_from_object(7), // space
            Self::char_from_undefined(0), // u
            Self::char_from_undefined(1), // n
            Self::char_from_true(3), // e
            Self::char_from_false(3), // s
            Self::char_from_object(5), // c
            Self::char_from_false(1), // a
            Self::char_from_radix36(25), // p
            Self::char_from_true(3) // e
        );
        let unescape_func = format!("{}({})()", Self::get_function_constructor(), unescape_code);
        
        let escape_code = format!("([]+{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{} +{})",
            Self::char_from_true(1), // r
            Self::char_from_true(3), // e
            Self::char_from_true(0), // t
            Self::char_from_undefined(0), // u
            Self::char_from_true(1), // r
            Self::char_from_undefined(1), // n
            Self::char_from_object(7), // space
            Self::char_from_true(3), // e
            Self::char_from_false(3), // s
            Self::char_from_object(5), // c
            Self::char_from_false(1), // a
            Self::char_from_radix36(25), // p
            Self::char_from_true(3) // e
        );
        let escape_func = format!("{}({})()", Self::get_function_constructor(), escape_code);
        
        let percent = format!("({}({})+[])[+[]]", escape_func, Self::char_from_object(0));
        
        let hex = format!("{:02x}", c as u8);
        let mut hex_parts = vec![percent];
        for h in hex.chars() {
            hex_parts.push(Self::encode_char(h));
        }
        
        format!("{}({})", unescape_func, hex_parts.join("+"))
    }
}
