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

    pub fn fib() -> impl Iterator<Item = u64> {
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
                    return Some(i)
                }
            }
        }
    }

    pub fn is_prime(i: u64) -> bool {
        let upper_bound = (i as f64).sqrt() as u64 + 1;
        (2..upper_bound).all(|j| i % j != 0) || i == 2
    }

    pub fn primes() -> impl Iterator<Item = u64> {
        Primes(2)
    }
}

pub use fib::fib;
pub use primes::{primes, is_prime};