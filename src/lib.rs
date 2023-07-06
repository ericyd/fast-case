use wasm_bindgen::prelude::*;

mod camel_case;
mod snake_case;
mod util;

#[wasm_bindgen]
pub fn convert_case(case: &str, s: &str) -> String {
    match case {
        "snake_case" => snake_case::to_snake_case(s),
        "camelCase" => camel_case::to_camel_case(s),
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
