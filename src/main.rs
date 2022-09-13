fn main() {
    let primes = make_primes(10);
    println!("{primes:?}");
}

fn make_primes(count: usize) -> Vec<u32> {
    let mut primes = Vec::new();
    if count <= 0 {
        return primes;
    }
    primes.push(2);
    let mut candidate = 3;
    'outer: while primes.len() < count {
        for p in &primes {
            if p*p > candidate {
                continue;
            }
            if candidate % p == 0 {
                candidate += 2;
                continue 'outer;
            }
        }
        primes.push(candidate);
        candidate += 2;
    }
    primes
}
