fn main() {
    let primes = make_primes(1_000_000_000);
    println!("{primes:?}");
}

fn make_primes(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    primes.push(2);
    'search: for candidate in (3..=limit).step_by(2) {
        for p in &primes {
            if p * p > candidate {
                break;
            }
            if candidate % p == 0 {
                continue 'search;
            }
        }
        primes.push(candidate);
    }
    primes
}
