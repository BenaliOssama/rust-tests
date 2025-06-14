pub fn first_subword(mut s: String) -> String {
    //camelCase, PascalCase, and snake_case
    for (i, char) in s.char_indices() {
        if (char == '_' || char.is_uppercase() )&& i != 0 {
            return s[..i].to_string();
        }
    }
    
    s[..].to_string()
}
