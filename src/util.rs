pub fn convert_case(
    s: &str,
    delimiter: Option<char>,
    transformer: fn(char) -> char,
    uppercase_first_char: bool,
    uppercase_word_boundary: bool,
) -> String {
    let chars = s.chars().collect::<Vec<char>>();
    // This will be larger than we need, but it will avoid re-allocations.
    // If every 2-char non-delimiter sequence is a word boundary, then we would need 1.5x the size
    // (2 chars + (maybe) 1 inserted delimiter).
    // Delimiter chars will either be removed or replaced with an equivalently-sized delimiter char,
    // and same with letters.
    let mut result: Vec<char> = Vec::with_capacity(s.len() * 2);
    // `capitalize` dynamically flips based on word boundary.
    let mut capitalize = uppercase_first_char;
    for i in 0..chars.len() {
        if chars[i] == ' ' || chars[i] == '-' || chars[i] == '_' {
            capitalize = uppercase_word_boundary;
            if let Some(char) = delimiter {
                result.push(char);
            }
            continue;
        }

        if i > 0
            && chars[i].is_uppercase()
            && (chars[i - 1].is_lowercase()
                || (i < chars.len() - 1
                    && chars[i - 1].is_uppercase()
                    && chars[i + 1].is_lowercase()))
        {
            if let Some(char) = delimiter {
                result.push(char);
            }
            capitalize = uppercase_word_boundary;
        }

        if capitalize {
            capitalize = false;
            result.push(chars[i].to_uppercase().next().unwrap());
        } else {
            result.push(transformer(chars[i]));
        }
    }
    result.iter().collect::<String>()
}

#[inline]
pub fn to_lowercase(c: char) -> char {
    c.to_lowercase().next().unwrap()
}

#[inline]
pub fn to_uppercase(c: char) -> char {
    c.to_uppercase().next().unwrap()
}
