use crate::util;

pub fn run() -> u64 {
    util::primes().nth(10000).unwrap()
}