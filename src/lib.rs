use bitvec::prelude::*;
use core::iter::*;

// WARNING: not very accurate.
#[inline(always)]
fn isqrt(x: usize) -> usize {
    (x as f64).sqrt() as usize
}

pub fn make_primes(limit: usize) -> impl Iterator<Item = usize> {
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
    std::iter::once(2).chain(primes_without_2)
}
