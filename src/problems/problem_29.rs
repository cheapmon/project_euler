use std::collections::HashSet;
use num::BigInt;

pub(super) fn run() -> usize {
    let mut terms = HashSet::new();

    for a in 2..=100 {
        for b in 2..=100 {
            terms.insert(BigInt::from(a).pow(b));
        }
    }

    terms.len()
}