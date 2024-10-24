pub(super) fn run() -> u64 {
    let mut sum_of_squares = 0u64;
    let mut square_of_sum = 0u64;

    for i in 1..=100 {
        sum_of_squares += i * i;
        square_of_sum += i;
    }

    square_of_sum *= square_of_sum;
    square_of_sum - sum_of_squares
}