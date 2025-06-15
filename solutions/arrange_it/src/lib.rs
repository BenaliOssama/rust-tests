pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<String> = phrase.split_whitespace().map(|s| s.to_string()).collect();
    let mut map = std::collections::HashMap::new();

    for word in &words {
        let index = extract_number(word);
        map.insert(index, word.clone().replace(|c: char| c.is_digit(10), ""));
    }

    let mut result = String::new();
    for i in 1..=map.len() {
        result.push_str(&map[&i]);
        result.push(' ');
    }

    result.trim().to_string()
}

fn extract_number(s: &str) -> usize {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    digits.parse().unwrap()
}
