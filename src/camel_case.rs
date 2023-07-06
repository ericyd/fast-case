use super::util::split_into_words;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = split_into_words(s);
    words
        .into_iter()
        .enumerate()
        .map(|(i, s)| {
            if i == 0 {
                s.to_ascii_lowercase()
            } else {
                let temp = s.to_ascii_lowercase();
                temp[0..1].to_ascii_uppercase() + &temp[1..]
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod camel_case_tests {
    use super::super::convert_case;
    const CASE: &str = "camelCase";
    const EXPECTED: &str = "testLots";

    #[test]
    fn identity() {
        assert_eq!(convert_case(CASE, EXPECTED), EXPECTED);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(convert_case(CASE, "testing"), "testing");
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
    fn snake_case() {
        assert_eq!(convert_case(CASE, "test_lots"), EXPECTED);
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
    fn sentence_case() {
        assert_eq!(convert_case(CASE, "Test lots"), EXPECTED);
    }

    #[test]
    fn kebab_case() {
        assert_eq!(convert_case(CASE, "test-lots"), EXPECTED);
    }
}
