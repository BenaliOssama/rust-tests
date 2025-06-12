pub fn rev_str(input: &str) -> String {
    let chars : Vec<char> = input.chars().collect();
    let mut s = String::new();

    for c in chars.iter().rev(){
        s.push(*c);
    }
    s
}
