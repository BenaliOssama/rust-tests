use std::collections::HashMap;

fn main() {
    let mut map1 = HashMap::new();
    map1.insert("hello", 1);
    map1.insert("world", 2);

    let mut map2 = HashMap::new();
    map2.insert("hello", 1);
    map2.insert("world", 2);

    let mut map3 = HashMap::new();
    map3.insert("hello", 1);
    map3.insert("world", 3);

    println!("{}", map1 == map2);  // prints: true
    println!("{}", map1 == map3);  // prints: false
}
