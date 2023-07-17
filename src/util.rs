pub fn split_into_words(s: &str) -> Vec<&str> {
    // this will likely be larger than we need, but it will avoid re-allocations.
    // We can safely divide by 2 because words will always be at least 2 chars;
    // even though this isn't linguistically true, it is true in our case because
    // we must have 2 chars to compare word boundaries
    let mut words: Vec<&str> = Vec::with_capacity(s.len() / 2);

    // the largest possible word is the full string length
    // let mut word: String = String::with_capacity(s.len());
    let mut slice_start: usize = 0;
    let mut slice_end: usize = 0;
    let mut chars = s.chars();
    for (i, letter) in chars.clone().into_iter().enumerate() {
        if slice_end > slice_start {
            // whitespace, underscore, and hyphens are always a word break
            if letter.is_whitespace() || letter == '-' || letter == '_' {
                words.push(&s[slice_start..slice_end]);
                slice_end += 1;
                slice_start = slice_end;
            }
            // if letter is uppercase and previos letter is lowercase, that is a word break
            else if letter.is_ascii_uppercase() {
                match chars.nth(i - 1) {
                    Some(char) => {
                        if char.is_ascii_lowercase() {
                            words.push(&s[slice_start..slice_end]);
                            slice_end += 1;
                            // must include the current (uppercase) char in the slice
                            slice_start = slice_end - 1;
                        } else {
                            slice_end += 1;
                        }
                    }
                    None => slice_end += 1,
                }
            } else {
                slice_end += 1;
            }
        } else {
            slice_end += 1;
        }
    }
    words.push(&s[slice_start..slice_end]);
    words
}

pub fn convert_case(
    s: &str,
    delimiter: Option<char>,
    transformer: fn(char) -> char,
    uppercase_first_char: bool,
    uppercase_word_boundary: bool,
) -> String {
    let chars = s.chars().collect::<Vec<char>>();
    let mut result: Vec<char> = Vec::with_capacity(s.len() * 2);
    // capitalize flips based on word boundary
    let mut capitalize = uppercase_first_char;
    for i in 0..chars.len() {
        if chars[i] == ' ' || chars[i] == '-' || chars[i] == '_' {
            // capitalize after a word boundary if needed
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

pub fn to_lowercase(c: char) -> char {
    c.to_lowercase().next().unwrap()
}

pub fn to_lowercase_string_vec(s: &str) -> Vec<String> {
    let words: Vec<&str> = split_into_words(s);
    words
        .into_iter()
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<String>>()
}

pub fn to_capitalized_string_vec(s: &str) -> Vec<String> {
    let words: Vec<&str> = split_into_words(s);
    words
        .into_iter()
        .map(|s| {
            let temp = s.to_ascii_lowercase();
            temp[0..1].to_ascii_uppercase() + &temp[1..]
        })
        .collect::<Vec<String>>()
}
