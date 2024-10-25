use std::collections::HashSet;
use super::problem_21::proper_divisors;

pub(super) fn run() -> u64 {
    let abundant_numbers: Vec<u64> = (1..=28123).filter(|i| is_abundant(*i)).collect();
    let mut all_numbers: HashSet<u64> = (1..=28123).collect();

    for i in 0..abundant_numbers.len() {
        for j in i..abundant_numbers.len() {
            let i = abundant_numbers[i];
            let j = abundant_numbers[j];

            all_numbers.remove(&(i + j));
        }
    }

    all_numbers.iter().sum()
}

fn is_abundant(i: u64) -> bool {
    proper_divisors(i).iter().sum::<u64>() > i
}