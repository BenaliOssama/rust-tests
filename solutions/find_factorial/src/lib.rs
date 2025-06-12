pub fn factorial(num: u64) -> u64 {
    let mut result : u64 = 1 ; 
    for n in 1..num {
        result = result*n 
    }
    result * num 
}
