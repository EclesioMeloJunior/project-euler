pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mul = (1..limit).filter(|&x| {
        factors.iter().any(|&j| x % j == 0)
    });

    mul.sum()
}