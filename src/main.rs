mod problem1;
mod problem2;

fn main() {
    println!("Multiples of 3 and 5: {}", problem1::sum_of_multiples(1000, &[3, 5]));
    println!("Even Fibonacci numbers: {}" ,problem2::sum_fib_even_numbers(4_000_000))
}

