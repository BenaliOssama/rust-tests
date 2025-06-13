pub fn is_empty(v: &str) -> bool {
    v.len() == 0 
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        if c < ' ' || c > '~' {
            return false ; 
        }
    }
    true
}

pub fn contains(v: &str, pat: &str) -> bool {
    let mut i: usize = 0 ; 
    let mut j: usize = pat.len() - 1 ;

    while j < v.len() {
        if v[i..=j] == *pat {
            return true ; 
        }
        i += 1 ; 
        j += 1 ; 
    }
    false
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (&v[..index] , &v[index..]) 
}



pub fn find(v: &str, pat: char) -> usize {
    let mut i : usize = 0 ; 
    for c in v.chars() {
        if c == pat  {
            return i ;
        }
        i += 1 ; 
    }
    v.len()
}
