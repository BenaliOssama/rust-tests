use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map : HashMap<String,u32> = HashMap::new();
    // let str : Vec<String> = words.split_whitespace();

    let chars : String = words.chars().filter(|x| *x != '“' && *x != '―').collect();

    let s4: Vec<_>  = chars.split_whitespace().collect();

    for word in s4 {
        if let Some(val) = map.get_mut(word) { 
            *val = *val + 1 ;
        } else {
            map.insert(word.to_string(), 1 as u32);
        }
        
    }

    map
}
