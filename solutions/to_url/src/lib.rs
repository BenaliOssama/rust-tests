pub fn to_url(s: &str) -> String {
    //s.replace(' ', "%20")
    //s.split_whitespace().collect::<Vec<_>>().join("&20")
    let mut new_s = String::new();

    for c in s.chars() {
        if c == ' ' {
            new_s.push('%');
            new_s.push('2');
            new_s.push('0');
        } else {
            new_s.push(c);
        }
    }
    new_s
}
