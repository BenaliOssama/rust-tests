fn main() {
    let vec = vec![String::from("Hello"), String::from("World")];
    for s in vec.into_iter() {
        println!("{}", s);
    }
}
