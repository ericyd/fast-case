use wasm_bindgen::prelude::*;

mod camel_case;
mod kebab_case;
mod pascal_case;
mod sentence_case;
mod snake_case;
mod title_case;
mod util;

#[wasm_bindgen]
pub fn convert_case(case: &str, s: &str) -> String {
    match case {
        "snake_case" => snake_case::to_snake_case(s),
        "camelCase" => camel_case::to_camel_case(s),
        "Sentence case" => sentence_case::to_sentence_case(s),
        "Title Case" => title_case::to_title_case(s),
        "PascalCase" => pascal_case::to_pascal_case(s),
        "kebab-case" => kebab_case::to_kebab_case(s),
        _ => {
            let err = format!("Unrecognized case: {}", case);
            error(err.clone());
            err
        }
    }
}

// commonjs... see README
// defines a function that allows Rust to call back to JS
#[wasm_bindgen(inline_js = "exports.error = function error(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: String);
}
