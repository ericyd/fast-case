// use super::util::split_into_words;
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// pub fn to_camel_case(s: &str) -> String {
//     let words: Vec<&str> = split_into_words(s);
//     words
//         .into_iter()
//         .enumerate()
//         .map(|(i, s)| {
//             if i == 0 {
//                 s.to_ascii_lowercase()
//             } else {
//                 let temp = s.to_ascii_lowercase();
//                 temp[0..1].to_ascii_uppercase() + &temp[1..]
//             }
//         })
//         .collect::<Vec<String>>()
//         .join("")
// }

use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_camel_case(s: &str) -> String {
    convert_case(s, None, to_lowercase, false, true)
}

#[cfg(test)]
mod camel_case_tests {
    use super::to_camel_case;
    const EXPECTED: &str = "testLots";

    // credit: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/shouty_kebab.rs#L51-L58
    macro_rules! test {
        ($test_name:ident : $str1:expr => $str2:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(to_camel_case($str1), $str2);
            }
        };
    }

    test!(identity: EXPECTED => EXPECTED);
    test!(single_lowercase_word: "testing" => "testing");
    test!(two_lowercase_words: "test lots" => EXPECTED);
    test!(two_uppercase_words: "TEST LOTS" => EXPECTED);
    // TODO: expected behavior?
    // test!(two_mixed_case_words: "Test LOts" => EXPECTED);
    test!(screaming_snake_case: "TEST_LOTS" => EXPECTED);
    test!(snake_case: "test_lots" => EXPECTED);
    test!(title_case: "Test Lots" => EXPECTED);
    test!(pascal_case: "TestLots" => EXPECTED);
    test!(sentence_case: "Test lots" => EXPECTED);
    test!(kebab_case: "test-lots" => EXPECTED);
}
