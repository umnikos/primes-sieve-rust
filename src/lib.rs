use bitvec::prelude::*;
use core::iter::*;
use no_panic::no_panic;

pub struct PrimeIterator<I: Iterator<Item = usize>> {
    primes: I,
    limit: usize,
}

impl<I: Iterator<Item = usize>> Iterator for PrimeIterator<I> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.primes.next()
    }
}

#[derive(Clone)]
pub struct PrimeChecker {
    primes: Vec<usize>,
    limit: usize,
}

impl<I: Iterator<Item = usize>> PrimeIterator<I> {
    pub fn into_checker(self) -> PrimeChecker {
        PrimeChecker {
            primes: self.primes.collect(),
            limit: self.limit,
        }
    }
}

impl PrimeChecker {
    #[no_panic]
    pub fn is_prime(&self, n: usize) -> Option<bool> {
        if self.limit >= n {
            Some(self.primes.binary_search(&n).is_ok())
        } else {
            let isqrt = ((n as f64).sqrt() as usize) + 5;
            for &p in &self.primes {
                if p > isqrt {
                    return Some(true);
                }
                if n % p == 0 {
                    return Some(false);
                }
            }
            None
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.primes.iter()
    }
}

pub fn make_primes(limit: usize) -> PrimeIterator<Box<dyn Iterator<Item = usize>>> {
    #[inline(always)]
    fn isqrt(x: usize) -> usize {
        (x as f64).sqrt() as usize
    }

    if limit < 2 {
        return PrimeIterator {
            primes: Box::from(std::iter::empty()),
            limit,
        };
    }

    let mut sieve = bitvec![1; limit+1];
    sieve.set(0, false);
    sieve.set(1, false);
    for i in (3..=isqrt(limit)).step_by(2) {
        if sieve[i] {
            for j in (i * i..=limit).step_by(2 * i) {
                sieve.set(j, false);
            }
        }
    }

    let primes_without_2 = sieve
        .into_iter()
        .enumerate()
        .skip(1)
        .step_by(2)
        .filter_map(|(i, x)| x.then_some(i));
    let primes = std::iter::once(2).chain(primes_without_2);

    PrimeIterator {
        primes: Box::from(primes),
        limit,
    }
}
