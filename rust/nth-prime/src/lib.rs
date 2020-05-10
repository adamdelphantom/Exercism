fn is_prime(prime_so_far: &[u32], candidate: u32) -> bool {
    for p in prime_so_far {
        if p * p > candidate {
            return true;
        }
        if candidate % p == 0 {
            return false;
        }
    }
    true
}


pub fn nth(n: u32) -> u32 {
    let mut primes = vec![];
    for candidate in 2.. {
        if is_prime(&primes[..], candidate as u32) {
            if primes.len() == n as usize {
                return candidate;
            } else {
                primes.push(candidate);
            }
        }
    }
    unreachable!()
}