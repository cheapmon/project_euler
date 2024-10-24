pub(super) fn run() -> u64 {
    for i in 1..=1000 {
        for j in i..=1000 {
            for k in j..=1000 {
                if (i * i) + (j * j) == (k * k) && i + j + k == 1000 {
                    return i * j * k;
                }
            }
        }
    }

    unreachable!()
}