// use super::util::to_lowercase_string_vec;
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// pub fn to_kebab_case(s: &str) -> String {
//     to_lowercase_string_vec(s).join("-")
// }

use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_kebab_case(s: &str) -> String {
    convert_case(s, Some('-'), to_lowercase, false, false)
}

#[cfg(test)]
mod kebab_case_tests {
    use super::to_kebab_case;
    const EXPECTED: &str = "test-lots";

    #[test]
    fn identity() {
        assert_eq!(to_kebab_case(EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(to_kebab_case("test"), "test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(to_kebab_case("test lots"), EXPECTED);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(to_kebab_case("TEST LOTS"), EXPECTED);
    }

    // TODO: what is the expected behavior with this spec?
    // #[test]
    // fn two_mixed_case_words() {
    //     assert_eq!(to_pascal_case("Test LOts"), EXPECTED);
    // }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(to_kebab_case("TEST_LOTS"), EXPECTED);
    }

    #[test]
    fn camel_case() {
        assert_eq!(to_kebab_case("testLots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(to_kebab_case("TestLots"), EXPECTED);
    }

    #[test]
    fn sentence_case() {
        assert_eq!(to_kebab_case("Test lots"), EXPECTED);
    }

    #[test]
    fn title_case() {
        assert_eq!(to_kebab_case("Test Lots"), EXPECTED);
    }

    #[test]
    fn snake_case() {
        assert_eq!(to_kebab_case("test_lots"), EXPECTED);
    }
}
