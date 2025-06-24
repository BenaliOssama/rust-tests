pub fn prev_prime(nbr: u64) -> u64 {
    let mut current = nbr;

    while current > 1 {
        match is_prime(current) {
            Some(true) => return current,
            Some(false) => current -= 1,
            None => return 0,  // theoretically unreachable here
        }
    }

    0
}

fn is_prime(nbr: u64) -> Option<bool> {
    if nbr == 0 || nbr == 1 {
        return None;
    }

    let mut i = 2;
    while i * i <= nbr {
        if nbr % i == 0 {
            return Some(false);
        }
        i += 1;
    }

    Some(true)
}

