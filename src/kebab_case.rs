use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_kebab_case(s: &str) -> String {
    convert_case(s, Some('-'), to_lowercase, false, false)
}

#[cfg(test)]
mod kebab_case_tests {
    use super::to_kebab_case;
    const EXPECTED: &str = "test-lots";

    // credit: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/shouty_kebab.rs#L51-L58
    macro_rules! test {
        ($test_name:ident : $str1:expr => $str2:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(to_kebab_case($str1), $str2);
            }
        };
    }

    test!(identity: EXPECTED => EXPECTED);
    test!(single_lowercase_word: "test" => "test");
    test!(two_lowercase_words: "test lots" => EXPECTED);
    test!(two_uppercase_words: "TEST LOTS" => EXPECTED);
    // TODO: expected behavior?
    // test!(two_mixed_case_words: "Test LOts" => EXPECTED);
    test!(screaming_snake_case: "TEST_LOTS" => EXPECTED);
    test!(camel_case: "testLots" => EXPECTED);
    test!(pascal_case: "TestLots" => EXPECTED);
    test!(sentence_case: "Test lots" => EXPECTED);
    test!(title_case: "Test Lots" => EXPECTED);
    test!(snake_case: "test_lots" => EXPECTED);
    test!(latin_test_a: "ÀêÙý ÇË téõÑæã" => "àê-ùý-çë-téõ-ñæã");
    test!(unicode_a: "Per Martin-Löf" => "per-martin-löf");
    test!(unicode_b: "Löwe 老虎 Léopard" => "löwe-老虎-léopard");
    test!(unicode_c: "ΑΒ ΓΔ ΘΛ" => "αβ-γδ-θλ");
    test!(unicode_d: "❤️🧡💛💚💙💜" => "❤️🧡💛💚💙💜");
    test!(unicode_e: "Test 🗻∈🌏 Lots" => "test-🗻∈🌏-lots");
    test!(unicode_f: "Test🗻∈🌏Lots" => "test🗻∈🌏lots");
    // from heck: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/kebab.rs#L68-L69
    test!(test9: "XΣXΣ baﬄe" => "xσxσ-baﬄe");
    test!(test10: "XMLHttpRequest" => "xml-http-request");
}
