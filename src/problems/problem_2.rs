use num::{BigInt, ToPrimitive};
use crate::util;

pub(super) fn run() -> u64 {
    util::fib()
        .take_while(|i| *i < BigInt::from(4_000_000))
        .filter(|i| i % 2 == BigInt::from(0))
        .map(|i| i.to_u64().unwrap())
        .sum()
}