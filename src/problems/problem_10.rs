use crate::util;

pub fn run() -> u64 {
    util::primes().take_while(|prime| *prime < 2_000_000).sum()
}