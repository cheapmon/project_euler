mod fib {
    struct Fib {
        current: u64,
        previous: u64,
    }

    impl Iterator for Fib {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let tmp = self.current;
            self.current += self.previous;
            self.previous = tmp;

            Some(self.current)
        }
    }

    pub fn fib() -> impl Iterator<Item=u64> {
        Fib { current: 1, previous: 0 }
    }
}

mod primes {
    struct Primes(u64);

    impl Iterator for Primes {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let i = self.0;
                self.0 += 1;

                if is_prime(i) {
                    return Some(i);
                }
            }
        }
    }

    pub fn is_prime(i: u64) -> bool {
        let upper_bound = (i as f64).sqrt() as u64 + 1;
        (2..upper_bound).all(|j| i % j != 0) || i == 2
    }

    pub fn prime_factors(i: u64) -> Vec<u64> {
        let mut i = i;
        let mut factors = vec![];

        'outer: while i > 1 {
            for prime in primes().take_while(|p| *p <= i) {
                if i % prime == 0 {
                    factors.push(prime);
                    i /= prime;
                    continue 'outer;
                }
            }
        }

        factors
    }

    pub fn primes() -> impl Iterator<Item=u64> {
        Primes(2)
    }
}

mod triangle_numbers {
    struct Tri {
        i: u64,
        sum: u64,
    }

    impl Iterator for Tri {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            self.sum += self.i;
            self.i += 1;
            Some(self.sum)
        }
    }

    pub fn triangle_numbers() -> impl Iterator<Item=u64> {
        Tri { i: 1, sum: 0 }
    }
}

pub use fib::fib;
pub use primes::{primes, is_prime, prime_factors};
pub use triangle_numbers::triangle_numbers;