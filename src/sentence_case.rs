use super::util::to_lowercase_string_vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_sentence_case(s: &str) -> String {
    let result = to_lowercase_string_vec(s).join(" ");
    result[0..1].to_ascii_uppercase() + &result[1..]
}

#[cfg(test)]
mod sentence_case_tests {
    use super::super::convert_case;
    const CASE: &str = "Sentence case";
    const EXPECTED: &str = "Test lots";

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
    fn title_case() {
        assert_eq!(convert_case(CASE, "Test Lots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(convert_case(CASE, "TestLots"), EXPECTED);
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
