use num::BigInt;
use crate::util;

pub(super) fn run() -> u64 {
    let (index, _) = util::fib()
        .enumerate()
        .map(|(index, i)| (index + 2, i))
        .take_while(|(_, i)| count_number_of_digits(i.clone()) != 1_000)
        .last()
        .unwrap();

    (index + 1) as u64
}

fn count_number_of_digits(i: BigInt) -> u64 {
    let mut i = i;
    let mut count = 0;

    while i > BigInt::from(0) {
        count += 1;
        i /= 10;
    }

    count
}