use super::*;
use std::ops::Deref;

impl Deref for PrimeVec {
    type Target = Vec<usize>;
    fn deref<'a>(&'a self) -> &'a Vec<usize> {
        &self.primes
    }
}

impl From<PrimeIter> for PrimeVec {
    fn from(x: PrimeIter) -> Self {
        PrimeVec {
            primes: x.primes.collect(),
            limit: x.limit,
        }
    }
}

impl From<PrimeSieve> for PrimeIter {
    fn from(x: PrimeSieve) -> PrimeIter {
        x.into_iter()
    }
}
