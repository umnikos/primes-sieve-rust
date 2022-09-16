use core::iter::*;
use bitvec::prelude::*;

fn main() {
    let primes = make_primes(2_000_000_000);

    write_to_file(primes);
    println!("done.");
}

fn write_to_file<T: Iterator<Item=usize>>(primes: T) {
    use std::fs::*;
    use std::io::*;

    let file = File::create("primes.txt").expect("can't make file!");
    let mut file = BufWriter::new(file); // BufWriter wrapper for performance

    for p in primes {
        let s = p.to_string() + "\n";
        file.write(s.as_bytes()).expect("can't write to file!");
    }

}

fn make_primes(limit: usize) -> impl Iterator<Item=usize> {
    let mut sieve = bitvec![1; limit+1];
    sieve.set(0,false);
    sieve.set(1,false);
    for i in (3..=limit).step_by(2) {
        if sieve[i] {
            for j in (i*i..=limit).step_by(2*i) {
                sieve.set(j,false);
            }
        }
    }

    let primes_without_2 =
        sieve
            .into_iter()
            .enumerate()
            .skip(1)
            .step_by(2)
            .filter_map(#[inline(always)] |(i, x)|
                x.then_some(i));
    std::iter::once(2).chain(primes_without_2)
}
