use std::collections::HashMap;

pub fn arrange_phrase(phrase: &str) -> String {
    let mut map = HashMap::new();

    // change into Vec<String>
    let v = str_to_string(phrase);

    // change into map int String
    for line in v {
        let (index, word) = extract_number(&line);
        map.insert(index, word);
    }
    let mut result = String::new();
    let mut i = 0;
    while map.len() != result.split_whitespace().count() {
        if let Some(value) = map.get(&i) {
            result.push_str(value);
            result.push(' ');
        }
        i += 1;
    }
    result.pop();
    result
}

fn extract_number(s: &str) -> (i32, String) {
    let digits: String = s.chars().filter(|c| c.is_digit(10)).collect();
    let non_digits: String = s.chars().filter(|c| !c.is_digit(10)).collect();

    (digits.parse().unwrap(), non_digits)
}

fn str_to_string(s: &str) -> Vec<String> {
    s.split_whitespace().map(|s| s.to_string()).collect()
}
