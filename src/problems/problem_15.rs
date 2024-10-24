use num::{BigInt, ToPrimitive};

pub(super) fn run() -> u64 {
    ((21..=40).product::<BigInt>() / (2..=20).product::<BigInt>()).to_u64().unwrap()
}