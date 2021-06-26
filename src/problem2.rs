pub fn sum_fib_even_numbers(n: i64) -> i64 {
    let mut x: (i64, i64) = (0, 2);
    let mut t = 2;

    let mut sum  = 0;

    while t < n {
        sum += t;
        t = x.0 + 4 * x.1;
        x = (x.1, t);
    }

    sum
}
