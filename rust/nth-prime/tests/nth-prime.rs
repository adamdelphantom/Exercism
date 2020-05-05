use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_fiftieth_prime() {
    assert_eq!(np::nth(49), 229);
}

#[test]
fn test_hundredth_prime() {
    assert_eq!(np::nth(99), 541);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}
