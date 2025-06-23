use std::collections::HashMap;


fn main() {
    println!("Hello, world!");
}


fn get_or_insert(
        map: &mut HashMap<u32, String>,
    )-> &String {
    match map.get(&22) {
        Some(v) => v,
        None => {
            map.insert(22,String::from("hi"));
            &map[&22]
        }
    }
}
