use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut map : HashMap<String, u32> = HashMap::new();
    let words: Vec<String> = words.split_whitespace().map(|s| {
        // Str
        return s.chars().filter(|c| (*c).is_alphanumeric() || (*c) == '\'' ).collect::<String>().to_lowercase();
    }).collect();
    for mut word in words {
        // if word != ""{
        //     if word.contains('\''){
        //         let ss = word.split('\'').collect::<Vec<&str>>();//.collect::<Vec<&str>>().join("\'");
        //         word = format!("{}\\'{}", ss[0], ss[1]);//ss[0].to_owned() + "\\'" + &ss[1];
        //     }
            let count = map.entry(word).or_insert(0);
            // *count += 1 ;
        }
    }
    map
}
