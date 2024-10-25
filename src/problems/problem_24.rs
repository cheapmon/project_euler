use std::collections::VecDeque;

pub(super) fn run() -> u64 {
    let mut n = (1..10).rev().map(|i| (1..=i).product::<u64>()).collect::<VecDeque<_>>();
    let mut i = 1_000_000 - 1;
    let mut digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut result = String::from("");

    while let Some(a) = n.pop_front() {
        let index = i / a;
        i -= index * a;
        result += &format!("{}", digits.remove(index as usize));
    }
    result += &format!("{}", digits.remove(0));

    result.parse().unwrap()
}