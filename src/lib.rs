use wasm_bindgen::prelude::*;

mod camel_case;
mod kebab_case;
mod pascal_case;
mod sentence_case;
mod snake_case;
mod title_case;
mod util;

pub use camel_case::to_camel_case;
pub use kebab_case::to_kebab_case;
pub use pascal_case::to_pascal_case;
pub use sentence_case::to_sentence_case;
pub use snake_case::to_snake_case;
pub use title_case::to_title_case;

// Why commonjs? see README
// This defines a function that allows Rust to call back to JS.
#[wasm_bindgen(inline_js = "exports.error = function error(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: String);
}
