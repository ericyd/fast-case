use super::util::to_capitalized_string_vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_title_case(s: &str) -> String {
    to_capitalized_string_vec(s).join(" ")
}

#[cfg(test)]
mod title_case_tests {
    use super::to_title_case;
    const EXPECTED: &str = "Test Lots";

    #[test]
    fn identity() {
        assert_eq!(to_title_case(EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(to_title_case("test"), "Test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(to_title_case("test lots"), EXPECTED);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(to_title_case("TEST LOTS"), EXPECTED);
    }

    #[test]
    fn two_mixed_case_words() {
        assert_eq!(to_title_case("Test LOts"), EXPECTED);
    }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(to_title_case("TEST_LOTS"), EXPECTED);
    }

    #[test]
    fn camel_case() {
        assert_eq!(to_title_case("testLots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(to_title_case("TestLots"), EXPECTED);
    }

    #[test]
    fn sentence_case() {
        assert_eq!(to_title_case("Test lots"), EXPECTED);
    }

    #[test]
    fn snake_case() {
        assert_eq!(to_title_case("test_lots"), EXPECTED);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(to_title_case("test-lots"), EXPECTED);
    }
}
