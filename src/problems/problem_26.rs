use std::collections::HashSet;

pub(super) fn run() -> u64 {
    (2..1000).max_by(|i, j| long_division(*i).len().cmp(&long_division(*j).len())).unwrap()
}

fn long_division(i: u64) -> Vec<u64> {
    let mut rest = 10;

    let mut digits = vec![];
    let mut divisions = HashSet::new();

    while !divisions.contains(&rest) && rest != 0 {
        divisions.insert(rest);

        while rest / i == 0 {
            digits.push(0);
            rest *= 10;
        }

        digits.push(rest / i);
        rest = (rest % i) * 10;
    }

    digits
}