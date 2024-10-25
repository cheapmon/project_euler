use crate::util::prime_factors;

pub(super) fn run() -> u64 {
    let d = |i: u64| proper_divisors(i).iter().sum::<u64>();
    let mut sum = 0;

    for i in 1..10_000 {
        let j = d(i);

        if d(j) == i && i != j {
            sum += i;
        }
    }

    sum
}

pub(super) fn proper_divisors(i: u64) -> Vec<u64> {
    let prime_factors = prime_factors(i);
    let mut proper_divisors = vec![1];

    for prime_factor in prime_factors {
        for j in 0..proper_divisors.len() {
            let product = proper_divisors[j] * prime_factor;

            if product != i {
                proper_divisors.push(product);
            }
        }
    }

    proper_divisors.sort();
    proper_divisors.dedup();

    proper_divisors
}