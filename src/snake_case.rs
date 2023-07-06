use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_snake_case(s: &str) -> String {
    let bytes = s.as_bytes();
    // this is more space than we need but will guarantee we avoid reallocations
    let mut result: Vec<u8> = Vec::with_capacity(bytes.len() * 2);
    let mut i = 0;
    while i < bytes.len() {
        // 32 == space
        // 45 == hyphen
        // 95 == underscore
        if bytes[i] == 32 || bytes[i] == 45 || bytes[i] == 95 {
            result.push(95); // underscore
            i += 1;
            continue;
        }

        // uppercase ascii; lowercase in place
        if bytes[i] >= 65 && bytes[i] <= 90 {
            // previous is lowercase ascii, i.e. word boundary
            if i > 0 && bytes[i - 1] >= 97 && bytes[i - 1] <= 122 {
                result.push(95); // underscore
                result.push(bytes[i] + 32);
            } else {
                result.push(bytes[i] + 32);
            }
            i += 1;
            continue;
        }

        result.push(bytes[i]);
        i += 1;
    }

    match String::from_utf8(result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
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
