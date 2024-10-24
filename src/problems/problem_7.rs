use crate::util;

pub(super) fn run() -> u64 {
    util::primes().nth(10000).unwrap()
}