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
    if n == 1 {
        return prime_factors;
    }
    if is_prime(&n) {
        prime_factors.push(n);
        return prime_factors;
    }
    for i in 2..=n {
        if is_prime(&i) && is_factor(&n, &i) {
            let mut remainder = n / i;
            prime_factors.push(i);
            if is_prime(&remainder) {
                prime_factors.push(remainder);
            }
            while !is_prime(&remainder) {    
                prime_factors.push(i);
                remainder = remainder / i;
                if is_prime(&remainder) {
                    prime_factors.push(remainder);
                }
            }
        }
    }  
    prime_factors
}


// TODO: need a way to find factorials of the same value maybe recursion?
