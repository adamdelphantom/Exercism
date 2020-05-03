pub fn nth(n: u32) -> u32 {
    // Iterate upwards collecting primes
    // in a vector. Until the length of 
    // the vector equals n. 
    let mut primes = vec![];
    let mut prime_candidate = 1;
    while primes.len() < n as usize{
        for i in prime_candidate..1 {
            if prime_candidate % i == 0 {
                prime_candidate += 1;
                break
            }
            primes.push(prime_candidate)
        }
        prime_candidate += 1;
    }
    //
    // to check for prime:
    // for i in prime_candidate..2 {
    //  if prime_candidate % i == 0 {
    //  not a prime   
    // }     
    // if all numbers do not mod 0  then
    //  prime.push(prime_candidate)
    // }

    primes[n as usize]
}