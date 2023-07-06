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
    const case: &str = "camelCase";
    const expected: &str = "testLots";

    #[test]
    fn identity() {
        assert_eq!(convert_case(case, "testLots"), expected);
    }

    #[test]
    fn single_lowercase_word() {
        assert_eq!(convert_case(case, "testing"), "testing");
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
    fn snake_case() {
        assert_eq!(convert_case(case, "test_lots"), expected);
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
