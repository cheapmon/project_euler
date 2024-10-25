use num::{BigInt, ToPrimitive};

pub(super) fn run() -> u64 {
    let mut i: BigInt = (2..=100).product();
    let mut sum_of_digits = BigInt::from(0);

    while i != BigInt::from(0) {
        sum_of_digits += &i % 10;
        i /= 10;
    }

    sum_of_digits.to_u64().unwrap()
}