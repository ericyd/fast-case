use super::util::to_capitalized_string_vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_title_case(s: &str) -> String {
    to_capitalized_string_vec(s).join(" ")
}

#[cfg(test)]
mod title_case_tests {
    use super::super::convert_case;
    const CASE: &str = "Title Case";
    const EXPECTED: &str = "Test Lots";

    #[test]
    fn identity() {
        assert_eq!(convert_case(CASE, EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(convert_case(CASE, "test"), "Test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(convert_case(CASE, "test lots"), EXPECTED);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(convert_case(CASE, "TEST LOTS"), EXPECTED);
    }

    #[test]
    fn two_mixed_case_words() {
        assert_eq!(convert_case(CASE, "Test LOts"), EXPECTED);
    }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(convert_case(CASE, "TEST_LOTS"), EXPECTED);
    }

    #[test]
    fn camel_case() {
        assert_eq!(convert_case(CASE, "testLots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(convert_case(CASE, "TestLots"), EXPECTED);
    }

    #[test]
    fn sentence_case() {
        assert_eq!(convert_case(CASE, "Test lots"), EXPECTED);
    }

    #[test]
    fn snake_case() {
        assert_eq!(convert_case(CASE, "test_lots"), EXPECTED);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(convert_case(CASE, "test-lots"), EXPECTED);
    }
}
