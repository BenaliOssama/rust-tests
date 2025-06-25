pub fn reverse_it(v: i32) -> String {
    if v < 0 {
        let mut s : String =  v.to_string().chars().rev().collect();
        s.pop();

        return String::from("-")  + &s + &v.to_string()[1..] ;     
    }
    v.to_string().chars().rev().collect::<String>() + &v.to_string()
}
