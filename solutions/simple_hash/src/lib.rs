use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut Hz = HashMap::new();
    for word in words {
        let count = Hz.entry(*word).or_insert(0 as usize);
        *count += 1 ;
    }

    Hz
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count : usize = 0 ; 

    for (key, value) in frequency_count{
        if *value == 1 {
            count += 1 ;
     }
    }
    return count ;
}
