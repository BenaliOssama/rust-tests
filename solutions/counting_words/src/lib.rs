use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();

    words
        .split_whitespace()
        .map(|s| {
            // keep only alphanumeric and single quotes
            s.chars()
                .filter(|c| c.is_alphanumeric() || *c == '\'')
                .collect::<String>()
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .for_each(|word| {
            // remove leading/trailing apostrophes
            let word = word.trim_matches('\'').to_string();
            if !word.is_empty() {
                *map.entry(word).or_insert(0) += 1;
            }
        });

    map
}

