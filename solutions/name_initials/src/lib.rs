
pub fn initials(names: Vec<&str>) -> Vec<String> {
    let result: Vec<String> = names.iter()
        .map(|name| name_initial(name))
        .collect();
    result
}

fn name_initial(s: &str)-> String {

    let (first, last) = s.split_once(' ').unwrap();

    let initial = format!("{}. {}.", first.chars().next().unwrap(), last.chars().next().unwrap());
    
    String::from(initial)

}

