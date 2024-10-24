use num::{BigInt, ToPrimitive};

pub(super) fn run() -> u64 {
    let mut i = BigInt::from(2).pow(1000);
    let mut sum = BigInt::from(0);

    while i > 0.into() {
        sum += &i % 10;
        i /= 10;
    }

    sum.to_u64().unwrap()
}