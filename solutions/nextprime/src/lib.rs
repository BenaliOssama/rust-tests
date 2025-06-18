pub fn next_prime(nbr: u64) -> u64 {
    if nbr < 2 {
        return 2; 
    }

    if is_prime(nbr) { return nbr };

    let mut n = nbr.clone() ; 


    while !is_prime(n) {
        n += 1 ;
    }
    n 
}


pub fn is_prime(nbr: u64) -> bool{
    let mut i = 2 ; 
    while i*i <= nbr {
        if nbr % i ==  0 {
            return false ;
        }
        i += 1 ;
    }
    true
}
