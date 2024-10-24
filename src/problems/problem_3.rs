use crate::util;

pub fn run() -> u64 {
    let num: u64 = 600_851_475_143;
    let upper_bound: u64 = (num as f64).sqrt() as u64 + 1;

    (2..=upper_bound).rev().find(|i| util::is_prime(*i) && num % i == 0).unwrap()
}