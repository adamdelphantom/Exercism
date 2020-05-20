fn number_to_vec(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits = number_to_vec(num);
    let power = digits.len() as u32;
    let mut sum: u32 = 0;
    for i in digits {
        sum += i.pow(power)
    }
    if sum == num {
        return true;
    }
    false
}
