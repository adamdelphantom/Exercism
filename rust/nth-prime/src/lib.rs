pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut prime_candidate = 1;
    while primes.len() < n as usize + 1 {
        if prime_candidate == 1 {
            prime_candidate += 1;
        } else {
            for i in 2..prime_candidate {
                if prime_candidate % i == 0 && i != prime_candidate {
                    prime_candidate += 1;
                    break
                }
            }
        }
        primes.push(prime_candidate);
        prime_candidate += 1;
    }
    println!("{}", n as usize);
    primes[n as usize - 1]
}