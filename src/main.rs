fn main() {
    let primes = make_primes(1_000_000_000);
    println!("done.");
    drop(primes);
}

fn make_primes(limit: usize) -> Vec<usize> {
    let mut sieve = vec![true; limit];
    sieve[0] = false;
    for i in (2..limit).step_by(2) {
        if sieve[i] {
            for j in (2*i..limit).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for (i,&b) in sieve.iter().enumerate() {
        if b {
            primes.push(i+1);
        }
    }
    primes
}
