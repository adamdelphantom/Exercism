pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut limited_factors = vec![];
    for factor in factors {
        for i in 1..limit {
            if !limited_factors.contains(&(i * factor)) {
                if i * factor < limit {
                    limited_factors.push(i * factor);
                } else {
                    break;
                }
            }
        }
    }
    limited_factors.iter().sum()
}
