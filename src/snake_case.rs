use super::util::split_into_words;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_snake_case(s: &str) -> String {
    let words: Vec<&str> = split_into_words(s);
    words
        .into_iter()
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

#[cfg(test)]
mod snake_case_tests {
    use super::super::convert_case;
    const case: &str = "snake_case";
    const expected: &str = "test_lots";

    #[test]
    fn identity() {
        assert_eq!(convert_case(case, "test_lots"), expected);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(convert_case(case, "test"), "test");
    }

    #[test]
    fn two_lowercase_words() {
        assert_eq!(convert_case(case, "test lots"), expected);
    }

    #[test]
    fn two_uppercase_words() {
        assert_eq!(convert_case(case, "TEST LOTS"), expected);
    }

    #[test]
    fn two_mixed_case_words() {
        assert_eq!(convert_case(case, "Test LOts"), expected);
    }

    #[test]
    fn screaming_snake_case() {
        assert_eq!(convert_case(case, "TEST_LOTS"), expected);
    }

    #[test]
    fn camel_case() {
        assert_eq!(convert_case(case, "testLots"), expected);
    }

    #[test]
    fn title_case() {
        assert_eq!(convert_case(case, "Test Lots"), expected);
    }

    #[test]
    fn pascal_case() {
        assert_eq!(convert_case(case, "TestLots"), expected);
    }

    #[test]
    fn sentence_case() {
        assert_eq!(convert_case(case, "Test lots"), expected);
    }
}
