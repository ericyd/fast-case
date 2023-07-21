use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_snake_case(s: &str) -> String {
    convert_case(s, Some('_'), to_lowercase, false, false)
}

#[cfg(test)]
mod snake_case_tests {
    use super::to_snake_case;
    const EXPECTED: &str = "test_lots";

    // credit: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/shouty_kebab.rs#L51-L58
    macro_rules! test {
        ($test_name:ident : $str1:expr => $str2:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(to_snake_case($str1), $str2);
            }
        };
    }

    test!(identity: EXPECTED => EXPECTED);
    test!(single_lowercase_word: "test" => "test");
    test!(two_lowercase_words: "test lots" => EXPECTED);
    test!(two_uppercase_words: "TEST LOTS" => EXPECTED);
    test!(mixed_case_words_a: "testLOTS" => EXPECTED);
    test!(mixed_case_words_b: "TESTLots" => EXPECTED);
    test!(screaming_snake_case: "TEST_LOTS" => EXPECTED);
    test!(camel_case: "testLots" => EXPECTED);
    test!(title_case: "Test Lots" => EXPECTED);
    test!(pascal_case: "TestLots" => EXPECTED);
    test!(sentence_case: "Test lots" => EXPECTED);
    test!(kebab_case: "test-lots" => EXPECTED);
    // upper ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖØÙÚÛÜÝÞ
    // lower àáâãäåæçèéêëìíîïðñòóôõöøùúûüýþ
    test!(latin_test_a: "ÀêÙý ÇË téõÑæã" => "àê_ùý_çë_téõ_ñæã");
    test!(unicode_a: "Per Martin-Löf" => "per_martin_löf");
    test!(unicode_b: "Löwe 老虎 Léopard" => "löwe_老虎_léopard");
    test!(unicode_c: "ΑΒ ΓΔ ΘΛ" => "αβ_γδ_θλ");
    test!(unicode_d: "❤️🧡💛💚💙💜" => "❤️🧡💛💚💙💜");
    test!(unicode_e: "Test 🗻∈🌏 Lots" => "test_🗻∈🌏_lots");
    test!(unicode_f: "Test🗻∈🌏Lots" => "test🗻∈🌏lots");
}
