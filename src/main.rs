
fn main() {
    let primes = make_primes(2_000_000_000);

    //write_to_file(primes);
    println!("done.");
}

fn write_to_file(primes : Vec<usize>) {
    use std::fs::*;
    use std::io::*;
    let mut file = File::create("primes.txt").expect("can't make file!");
    let mut out = String::new();
    for p in primes {
        let s = p.to_string();
        let s = s + "\n";
        out += &s;
    }
    file.write(out.as_bytes()).expect("can't write to file!");

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

    let mut primes = Vec::with_capacity(limit/2);
    for (i,&b) in sieve.iter().enumerate() {
        if b {
            primes.push(i+1);
        }
    }
    primes
}
