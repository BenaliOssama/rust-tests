pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let fragments: Vec<String> = input.split_whitespace().map(|x| capitalize_first(x)).collect();
    fragments.join(" ")
}



pub fn change_case(input: &str) -> String {
    let mut chars = input.chars();
    let mut new_chars: Vec<char> = Vec::new();

    loop {
        match chars.next() {
            None => break,
            Some(f) => {
                if f.is_uppercase() {
                    new_chars.extend(f.to_lowercase());
                } else if f.is_lowercase() {
                    new_chars.extend(f.to_uppercase());
                } else {
                    new_chars.push(f);
                }
            }
        }
    }

    new_chars.into_iter().collect()
}
