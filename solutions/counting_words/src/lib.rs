use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map : HashMap<String, u32> = HashMap::new();
    let words: Vec<String> = words.split_whitespace().map(|s| {
        // Str
        return s.chars().filter(|c| (*c).is_alphanumeric() || (*c) == '\'' ).collect::<String>().to_lowercase();
    }).collect();
    for mut word in words {
        if word != ""{
            word = word.trim_matches('\'').to_string();
            let count = map.entry(word).or_insert(0);
            *count += 1 ;
        }
    }
    map
}
