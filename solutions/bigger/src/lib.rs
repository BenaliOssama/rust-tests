use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max : i32 = 0 ;
    for (key, value) in h {
        if value > max {
            max = value 
        }
    }
    max
}
