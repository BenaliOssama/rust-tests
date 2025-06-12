fn main() {
    let var = 10;
    println!("{}", std::any::type_name_of_val(&var));
}
