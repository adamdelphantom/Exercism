pub fn nth(n: u32) -> u32 {
    let mut prime_counter = 1;
    let mut prime_candidate: u32 = 2;
    while prime_counter < n {   
        for i in 2..prime_candidate {
            if prime_candidate % i == 0 {
                prime_candidate += 1;
                break
            }
        }
        prime_candidate += 1;
        prime_counter += 1;
    }
    prime_candidate
    // TODO: find out why the big prime hunt is stopping at 20000
}