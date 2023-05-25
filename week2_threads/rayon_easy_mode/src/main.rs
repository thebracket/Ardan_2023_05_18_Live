use std::time::Instant;
use rayon::prelude::*;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).into_par_iter().all(|i| n % i != 0 )
}

fn main() {
    let now = Instant::now();
    let numbers: Vec<u64> = (2 .. 1_000_000).collect();
    let mut primes: Vec<&u64> = numbers.par_iter().filter(|&n| is_prime(*n as u32)).collect();
    primes.par_sort_unstable();
    let elapsed = now.elapsed();
    println!("{primes:?}");
    println!("It took {} ms to find {} primes", elapsed.as_millis(), primes.len());
}
