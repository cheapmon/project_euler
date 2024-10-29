pub(super) fn run() -> u64 {
    let mut n = 1;
    let mut gap = 2;
    let mut sum = 1;
    let width = 1001;

    'outer: loop {
        for _ in 0..4 {
            n += gap;

            if n > width * width {
                break 'outer;
            }

            sum += n;
        }

        gap += 2;
    }

    sum
}