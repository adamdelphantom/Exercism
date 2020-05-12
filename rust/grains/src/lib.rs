pub fn square(s: u32) -> u64 {
    
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    let mut grains: u64 = 1;
    for _i in 2..=s {
        grains *= 2;
    }
    grains
}

pub fn total() -> u64 {
    18_446_744_073_709_551_615
}
