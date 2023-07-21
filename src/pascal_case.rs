use super::util::{convert_case, to_lowercase};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn to_pascal_case(s: &str) -> String {
    convert_case(s, None, to_lowercase, true, true)
}

#[cfg(test)]
mod pascal_case_tests {
    use super::to_pascal_case;
    const EXPECTED: &str = "TestLots";

    // credit: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/shouty_kebab.rs#L51-L58
    macro_rules! test {
        ($test_name:ident : $str1:expr => $str2:expr) => {
            #[test]
            fn $test_name() {
                assert_eq!(to_pascal_case($str1), $str2);
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
    test!(title_case: "Test Lots" => EXPECTED);
    test!(sentence_case: "Test lots" => EXPECTED);
    test!(snake_case: "test_lots" => EXPECTED);
    test!(kebab_case: "test-lots" => EXPECTED);
    test!(latin_test_a: "ÀêÙý_ÇË_téõÑæã" => "ÀêÙýÇëTéõÑæã");
    test!(unicode_a: "Per_Martin_Löf" => "PerMartinLöf");
    test!(unicode_b: "Löwe_老虎_Léopard" => "Löwe老虎Léopard");
    test!(unicode_c: "ΑΒ_ΓΔ_ΘΛ" => "ΑβΓδΘλ");
    test!(unicode_d: "❤️🧡💛💚💙💜" => "❤️🧡💛💚💙💜");
    test!(unicode_e: "Test 🗻∈🌏 Lots" => "Test🗻∈🌏Lots");
    // This one looks a bit strange, but because of the "uppercase-ness" of the middle unicode chars, it is just the way it works
    test!(unicode_f: "Test🗻∈🌏Lots" => "Test🗻∈🌏lots");
    // from heck: https://github.com/withoutboats/heck/blob/76a8274f948fbe3551413dc09026b733aca71995/src/upper_camel.rs#L82-L83
    test!(test9: "XΣXΣ baﬄe" => "XσxσBaﬄe");
    test!(test10: "XMLHttpRequest" => "XmlHttpRequest");
}
