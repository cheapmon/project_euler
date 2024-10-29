pub(super) fn run() -> u64 {
    let mut number_of_solutions = 0;

    for a in 0..=200 {
        let rest = compute_rest([a, 0, 0, 0, 0, 0, 0, 0]);
        let max = rest / 2;
        if rest == 0 {
            number_of_solutions += 1;
            continue;
        }

        for b in 0..=max {
            let rest = compute_rest([a, b, 0, 0, 0, 0, 0, 0]);
            let max = rest / 5;
            if rest == 0 {
                number_of_solutions += 1;
                continue;
            }

            for c in 0..=max {
                let rest = compute_rest([a, b, c, 0, 0, 0, 0, 0]);
                let max = rest / 10;
                if rest == 0 {
                    number_of_solutions += 1;
                    continue;
                }

                for d in 0..=max {
                    let rest = compute_rest([a, b, c, d, 0, 0, 0, 0]);
                    let max = rest / 20;
                    if rest == 0 {
                        number_of_solutions += 1;
                        continue;
                    }

                    for e in 0..=max {
                        let rest = compute_rest([a, b, c, d, e, 0, 0, 0]);
                        let max = rest / 50;
                        if rest == 0 {
                            number_of_solutions += 1;
                            continue;
                        }

                        for f in 0..=max {
                            let rest = compute_rest([a, b, c, d, e, f, 0, 0]);
                            let max = rest / 100;
                            if rest == 0 {
                                number_of_solutions += 1;
                                continue;
                            }

                            for g in 0..=max {
                                let rest = compute_rest([a, b, c, d, e, f, g, 0]);
                                let max = rest / 200;
                                if rest == 0 {
                                    number_of_solutions += 1;
                                    continue;
                                }

                                for h in 0..=max {
                                    let rest = compute_rest([a, b, c, d, e, f, g, h]);
                                    if rest == 0 {
                                        number_of_solutions += 1;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    number_of_solutions
}

fn compute_rest(v: [u64; 8]) -> u64 {
    let [a, b, c, d, e, f, g, h] = v;
    200 - (a * 1) - (b * 2) - (c * 5) - (d * 10) - (e * 20) - (f * 50) - (g * 100) - (h * 200)
}