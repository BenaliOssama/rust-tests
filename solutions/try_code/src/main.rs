fn main(){
    let arr = [1, 2, 3, 4];

    let sum = arr.iter().copied().reduce(|a, b| a + b);

    println!("{:?}", sum.unwrap()); // Some(10)
}
