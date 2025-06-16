fn extract_number(s: &str) -> i32 {
    s.chars()
     .filter(|c| c.is_digit(10))
     .collect::<String>()
     .parse()
     .unwrap()
}

fn main() {
    let s = "hello 42 world";
    let num = extract_number(s);
    println!("{}", num);
}
