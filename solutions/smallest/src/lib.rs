use std::collections::HashMap;

pub fn smallest(h: HashMap<&str, i32>) -> i32 {
    if h.len() == 0 {
        return i32::MAX;
    }

    let mut smalest: i32 = *h.values().next().unwrap();

    for (_key, value) in h {
        if value < smalest {
            smalest = value;
        }
    }
    smalest
}
