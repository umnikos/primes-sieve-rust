fn main() {
    let primes = make_primes(10);
    println!("{primes:?}");
}

fn make_primes(count: usize) -> Vec<u32> {
    let mut primes = Vec::new();
    if count == 0 {
        return primes;
    }
    primes.push(2);
    'search: for candidate in (3..).step_by(2) {
        if primes.len() == count {
            break;
        }
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
