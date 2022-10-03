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

    // 0 -> 1
    // 1 -> 3
    // 2 -> 5
    // ......
    #[inline(always)]
    fn to_index(x: usize) -> usize {
        (x - 1) / 2
    }
    #[inline(always)]
    fn from_index(x: usize) -> usize {
        x * 2 + 1
    }

    let mut sieve = bitvec![1; to_index(limit)+1];

    for i in to_index(3)..=to_index(isqrt(limit)) {
        if sieve[i] {
            for j in (to_index(from_index(i).pow(2))..=to_index(limit)).step_by(from_index(i)) {
                sieve.set(j, false);
            }
        }
    }

    let primes_without_2 = sieve
        .into_iter()
        .enumerate()
        .skip(1)
        .filter_map(|(i, x)| x.then_some(from_index(i)));
    let primes = std::iter::once(2).chain(primes_without_2);

    PrimeIterator {
        primes: Box::from(primes),
        limit,
    }
}
