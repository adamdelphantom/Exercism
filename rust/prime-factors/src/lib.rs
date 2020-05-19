fn is_prime(n: u64) -> bool {
    for i in (2..n).rev() {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_factor(n: u64, factor: u64) -> bool {
    if n % factor == 0 {
        return true;
    }
    false
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut prime_factors = vec![];
    if n <= 1 {
        return prime_factors;
    } else if is_prime(n) {
        prime_factors.push(n);
        return prime_factors;
    } else {
        for i in 2..=n {
            if is_prime(i) && is_factor(n, i) {
                prime_factors.push(i);
                let mut sub_factors = factors(n / i);
                prime_factors.append(&mut sub_factors);
                return prime_factors;
            }
        }
    }
    unreachable!()
}
