//! A library for generating and using large arrays of prime numbers.

use bitvec::prelude::*;
use core::iter::*;

mod conversions;
mod helper;
use helper::*;

pub struct PrimeSieve {
    primes: BitVec,
    limit: usize,
}

impl IntoIterator for PrimeSieve {
    type Item = usize;
    type IntoIter = PrimeIter;
    fn into_iter(self) -> PrimeIter {
        if self.limit < 2 {
            return PrimeIter {
                primes: Box::from(std::iter::empty()),
                limit: self.limit,
            };
        }
        let primes_without_2 = self
            .primes
            .into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(i, x)| x.then_some(from_index(i)));
        let primes = std::iter::once(2).chain(primes_without_2);
        PrimeIter {
            primes: Box::new(primes),
            limit: self.limit,
        }
    }
}

/// An iterator over some number of primes.
pub struct PrimeIter {
    primes: Box<dyn Iterator<Item = usize>>,
    limit: usize,
}

impl Iterator for PrimeIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        self.primes.next()
    }
}

/// A vector containing some number of primes.
#[derive(Clone)]
pub struct PrimeVec {
    primes: Vec<usize>,
    limit: usize,
}
impl PrimeIter {
    pub fn into_prime_vec(self) -> PrimeVec {
        PrimeVec {
            primes: self.primes.collect(),
            limit: self.limit,
        }
    }
}

pub trait PrimeChecker {
    fn is_prime(&self, n: usize) -> Option<bool>;
}

impl PrimeChecker for PrimeVec {
    /// Use the primes in the vector to check whether a given number is prime.
    fn is_prime(&self, n: usize) -> Option<bool> {
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
}

impl PrimeChecker for PrimeSieve {
    fn is_prime(&self, n: usize) -> Option<bool> {
        if n < 2 {
            Some(false)
        } else if n == 2 {
            Some(true)
        } else if self.limit >= n {
            Some(self.primes[to_index(n)])
        } else {
            None
        }
    }
}

impl PrimeSieve {
    /// Create an iterator over the primes from 1 to N.
    /// Implemented using a sieve of Eratosthenes, may take a while.
    pub fn new(limit: usize) -> Self {
        if limit < 2 {
            return PrimeSieve {
                primes: bitvec![0;0],
                limit,
            };
        }

        let mut sieve = bitvec![1; to_index(limit)+1];

        for i in to_index(3)..=to_index(isqrt(limit)) {
            if sieve[i] {
                for j in (to_index(from_index(i).pow(2))..=to_index(limit)).step_by(from_index(i)) {
                    sieve.set(j, false);
                }
            }
        }

        PrimeSieve {
            primes: sieve,
            limit,
        }
    }
}

pub fn make_primes(limit: usize) -> PrimeIter {
    PrimeSieve::new(limit).into_iter()
}
