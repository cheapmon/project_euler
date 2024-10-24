use crate::util;
use std::collections::HashMap;

pub fn run() -> u64 {
    fn count_factors(i: u64) -> u64 {
        let mut factors: HashMap<u64, u64> = HashMap::new();

        for prime in util::prime_factors(i) {
            if factors.contains_key(&prime) {
                factors.insert(prime, factors[&prime] + 1);
            } else {
                factors.insert(prime, 1);
            }
        }

        factors.values().map(|i| i + 1).product()
    }

    util::triangle_numbers().find(|i| count_factors(*i) > 500).unwrap()
}