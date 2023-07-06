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
