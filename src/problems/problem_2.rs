use crate::util;

pub fn run() -> u64 {
    util::fib().take_while(|i| *i < 4_000_000).filter(|i| *i % 2 == 0).sum()
}