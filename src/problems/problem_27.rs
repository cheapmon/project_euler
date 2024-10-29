pub(super) fn run() -> i64 {
    let mut max = (0, 0, 0);

    for a in -999..=999 {
        for b in -1000..=1000 {
            let mut n = 0;

            while is_prime(quadratic(a, b, n)) {
                n += 1;
            }

            if n > max.2 {
                max = (a, b, n);
            }
        }
    }

    max.0 * max.1
}

fn is_prime(i: i64) -> bool {
    if i < 0 {
        false
    } else {
        crate::util::is_prime(i as u64)
    }
}

fn quadratic(a: i64, b: i64, n: i64) -> i64 {
    (n * n) + (a * n) + b
}