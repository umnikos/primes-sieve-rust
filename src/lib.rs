use bitvec::prelude::*;
use core::iter::*;

pub struct PrimeIterator<I: Iterator<Item = usize>> {
    primes: I,
    limit: usize,
}

impl<I: Iterator<Item = usize>> Iterator for PrimeIterator<I> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> { Some(self.primes.next()?) }
}

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
    pub fn is_prime(&self, n: usize) -> Option<bool> {
        let limit = self.limit;
        if limit * limit < n {
            return None;
        }
        if limit >= n {
            return Some(self.primes.binary_search(&n).is_ok());
        } else {
            None // TODO: implement
        }
    }
}



pub fn make_primes(limit: usize) -> PrimeIterator<impl Iterator<Item = usize>> {
    // WARNING: not very accurate.
    #[inline(always)]
    fn isqrt(x: usize) -> usize {
        (x as f64).sqrt() as usize
    }

    let mut sieve = bitvec![1; limit+1];
    sieve.set(0, false);
    sieve.set(1, false);
    for i in (3..=isqrt(limit) + 4).step_by(2) {
        if sieve[i] {
            for j in (i * i..=limit).step_by(2 * i) {
                sieve.set(j, false);
            }
        }
    }

    let primes_without_2 = sieve.into_iter().enumerate().skip(1).step_by(2).filter_map(
        #[inline(always)]
        |(i, x)| x.then_some(i),
    );
    let primes = std::iter::once(2).chain(primes_without_2);

    PrimeIterator {
        primes,
        limit,
    }
}
