// use super::util::to_lowercase_string_vec;
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// pub fn to_sentence_case(s: &str) -> String {
//     let result = to_lowercase_string_vec(s).join(" ");
//     result[0..1].to_ascii_uppercase() + &result[1..]
// }

use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_sentence_case(s: &str) -> String {
    convert_case(s, Some(' '), to_lowercase, true, false)
}

#[cfg(test)]
mod sentence_case_tests {
    use super::to_sentence_case;
    const EXPECTED: &str = "Test lots";

    #[test]
    fn identity() {
        assert_eq!(to_sentence_case(EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(to_sentence_case("test"), "Test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(to_sentence_case("test lots"), EXPECTED);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(to_sentence_case("TEST LOTS"), EXPECTED);
    }

    // TODO: what is the expected behavior with this spec?
    // #[test]
    // fn two_mixed_case_words() {
    //     assert_eq!(to_sentence_case("Test LOts"), EXPECTED);
    // }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(to_sentence_case("TEST_LOTS"), EXPECTED);
    }

    #[test]
    fn camel_case() {
        assert_eq!(to_sentence_case("testLots"), EXPECTED);
    }

    #[test]
    fn title_case() {
        assert_eq!(to_sentence_case("Test Lots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(to_sentence_case("TestLots"), EXPECTED);
    }

    #[test]
    fn snake_case() {
        assert_eq!(to_sentence_case("test_lots"), EXPECTED);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(to_sentence_case("test-lots"), EXPECTED);
    }
}
