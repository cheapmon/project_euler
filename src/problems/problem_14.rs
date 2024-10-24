use std::collections::HashMap;

pub fn run() -> u64 {
    let mut longest_chain = 1;
    let mut number_which_produces_longest_chain = 1;

    let mut cache: HashMap<u64, u64> = HashMap::new();
    cache.insert(1, 1);

    fn collatz(i: u64) -> u64 {
        if i % 2 == 0 {
            i / 2
        } else {
            3 * i + 1
        }
    }

    fn compute_length_of_chain(i: u64, cache: &mut HashMap<u64, u64>) -> u64 {
        if cache.contains_key(&i) {
            cache[&i]
        } else {
            let next = collatz(i);
            let length_of_chain = compute_length_of_chain(next, cache) + 1;

            cache.insert(i, length_of_chain);
            length_of_chain
        }
    }

    for i in 2..1_000_000 {
        let length_of_chain = compute_length_of_chain(i, &mut cache);

        if length_of_chain > longest_chain {
            longest_chain = length_of_chain;
            number_which_produces_longest_chain = i;
        }
    }

    number_which_produces_longest_chain
}