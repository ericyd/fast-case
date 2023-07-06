use super::util::to_lowercase_string_vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_snake_case(s: &str) -> String {
    to_lowercase_string_vec(s).join("_")
}

#[cfg(test)]
mod snake_case_tests {
    use super::to_snake_case;
    const EXPECTED: &str = "test_lots";

    #[test]
    fn identity() {
        assert_eq!(to_snake_case(EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(to_snake_case("test"), "test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(to_snake_case("test lots"), EXPECTED);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(to_snake_case("TEST LOTS"), EXPECTED);
    }

    #[test]
    fn two_mixed_case_words() {
        assert_eq!(to_snake_case("Test LOts"), EXPECTED);
    }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(to_snake_case("TEST_LOTS"), EXPECTED);
    }

    #[test]
    fn camel_case() {
        assert_eq!(to_snake_case("testLots"), EXPECTED);
    }

    #[test]
    fn title_case() {
        assert_eq!(to_snake_case("Test Lots"), EXPECTED);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(to_snake_case("TestLots"), EXPECTED);
    }

    #[test]
    fn sentence_case() {
        assert_eq!(to_snake_case("Test lots"), EXPECTED);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(to_snake_case("test-lots"), EXPECTED);
    }
}
