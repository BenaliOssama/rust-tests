fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    let result: Vec<String> = names.into_iter()
        .map(|name| name_initial(name))
        .collect();

    println!("{:?}", result);
}



fn name_initial(s: &str)-> String {

    let (first, last) = s.split_once(' ').unwrap();

    let initial = format!("{}. {}.", first.chars().next().unwrap(), last.chars().next().unwrap());
    
    String::from(initial)

}

