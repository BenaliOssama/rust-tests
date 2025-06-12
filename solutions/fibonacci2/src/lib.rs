pub fn fibonacci(n: u32) -> u32 {

    let mut  prev : u32  = 0 ; 
    let mut current : u32 = 1 ; 

    for _ in 0..n {

        let next = current + prev ;
        prev = current;
        current =  next ;  
    }
    prev
}

