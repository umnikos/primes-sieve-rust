use primes::make_primes;

fn main() {
    let primes = make_primes(2_000_000);

    write_to_file(primes);
    println!("done.");
}

fn write_to_file<T: Iterator<Item = usize>>(primes: T) {
    use std::fs::*;
    use std::io::*;

    let file = File::create("primes.txt").expect("can't make file!");
    let mut file = BufWriter::new(file); // BufWriter wrapper for performance

    for p in primes {
        let s = p.to_string() + "\n";
        file.write_all(s.as_bytes()).expect("can't write to file!");
    }
}
