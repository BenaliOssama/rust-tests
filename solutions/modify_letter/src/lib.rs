pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|c| *c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars().filter(|c| *c != letter.to_ascii_lowercase() && *c != letter.to_ascii_uppercase()).collect()
}
pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars().map(|c| {
        if c == letter.to_ascii_uppercase() ||  c == letter.to_ascii_lowercase() {
            if c.is_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c.to_ascii_uppercase()
            }
        } else {
            c
        }
    }).collect()
}

