fn is_prime(n: &u64) -> bool {
    for i in (2..*n).rev() {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_factor(n: &u64, factor: &u64) -> bool {
    if n % factor == 0 {
        return true;
    }
    false
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    let mut next: u64;
    for i in 2..=n {
       if is_factor(&n, &i) && is_prime(&i) {
           next = n / i;
           prime_factors.push(i);
           if is_prime(&next) && next > 1 {
            prime_factors.push(next);
        }
       } 
       
    }
    prime_factors
}

// TODO: need a way to find factorials of the same value maybe recursion?