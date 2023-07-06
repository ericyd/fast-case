use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(x: usize, y: usize) -> usize {
    x + y
}

// TODO: case should be enum
#[wasm_bindgen]
pub fn convert_case(case: &str,s: &str) -> () {
    
}

// so Rust can call back to JS
#[wasm_bindgen(inline_js = "export function error(s) { throw new Error(s) }")]
extern "C" {
    fn error(s: String);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}