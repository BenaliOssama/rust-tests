use std::collections::HashMap;


pub fn is_permutation(s1: &str, s2: &str) -> bool {

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for c in s1.chars() {
        let count = map1.entry(c).or_insert(0 as usize);
        *count += 1;
    }
    for c in s2.chars() {
        let count = map2.entry(c).or_insert(0 as usize);
        *count += 1;
    }

    map1 == map2
}
