use std::collections::HashMap;
//x̄ = (Σx) / n
pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        f64::NAN
    } else {
        let sum: f64 = list.iter().map(|x| *x as f64).sum();
        sum / list.len() as f64
    }
}

pub fn median(list: &[i32]) -> i32 {
    let mut copy = list.to_vec();
    copy.sort();
    let len = copy.len();
    if len % 2 == 0 {
        let mid1 = copy[len / 2 - 1] ;
        let mid2 = copy[len / 2] ;
        return (mid1 + mid2) / 2 as i32 ;
    } else {
        return copy[len / 2];
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;

    for &l in list {
        let count = map.entry(l).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = l;
        }
    }

    mode
}
