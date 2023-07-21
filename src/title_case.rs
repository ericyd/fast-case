use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_title_case(s: &str) -> String {
    convert_case(s, Some(' '), to_lowercase, true, true)
}

#[cfg(test)]
mod title_case_tests {
    use super::to_title_case;
    const EXPECTED: &str = "Test Lots";

    // credit: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/shouty_kebab.rs#L51-L58
    macro_rules! test {
        ($test_name:ident : $str1:expr => $str2:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(to_title_case($str1), $str2);
            }
        };
    }

    test!(identity: EXPECTED => EXPECTED);
    test!(single_lowercase_word: "test" => "Test");
    test!(two_lowercase_words: "test lots" => EXPECTED);
    test!(two_uppercase_words: "TEST LOTS" => EXPECTED);
    // TODO: expected behavior?
    // test!(two_mixed_case_words: "Test LOts" => EXPECTED);
    test!(screaming_snake_case: "TEST_LOTS" => EXPECTED);
    test!(camel_case: "testLots" => EXPECTED);
    test!(pascal_case: "TestLots" => EXPECTED);
    test!(sentence_case: "Test lots" => EXPECTED);
    test!(snake_case: "test_lots" => EXPECTED);
    test!(kebab_case: "test-lots" => EXPECTED);
}
