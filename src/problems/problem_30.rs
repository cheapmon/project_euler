pub(super) fn run() -> u64 {
    let mut sum = 0;

    for i in 10..1_000_000 {
        let mut j: u64 = i;
        let mut sum_of_fifth_powers = 0;

        while j != 0 {
            sum_of_fifth_powers += (j % 10).pow(5);
            j /= 10;
        }

        if i == sum_of_fifth_powers {
            sum += i;
        }
    }

    sum
}