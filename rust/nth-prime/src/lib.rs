pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::with_capacity(n as usize);
    let mut prime_candidate: u32 = 2;
    while primes.len() < n as usize + 1 {
        let mut is_prime = true;  
        if prime_candidate == 2 {
            primes.push(prime_candidate);
            prime_candidate += 1;
        } else {
            for i in 2..prime_candidate{
                if prime_candidate % i == 0 {
                    is_prime = false;
                }
            }
            if is_prime == false {
                prime_candidate += 1;
            } else {
                primes.push(prime_candidate);
                prime_candidate += 1;
            }
        }
    }
    primes[n as usize]
    // TODO: clean up perf
}