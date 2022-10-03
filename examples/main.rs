use inquire;
use primes::make_primes;
use std::time::Instant;

fn main() {
    let limit: usize = prompt_user_for_limit();
    let filename: String = prompt_user_for_filename();
    println!("Generating primes...");
    let start = Instant::now();

    let primes = make_primes(limit);
    write_to_file(primes, filename);

    let time_taken = start.elapsed();

    println!("Done! Took {} seconds", time_taken.as_secs_f32());
}

fn prompt_user_for_limit() -> usize {
    inquire::CustomType::<usize>::new("Generate primes up to:")
        .prompt()
        .unwrap()
}

fn prompt_user_for_filename() -> String {
    inquire::Text::new("Name of file to write to:")
        .with_default("primes.txt")
        .prompt()
        .unwrap()
}

fn write_to_file<T: Iterator<Item = usize>>(primes: T, filename: String) {
    use std::fs::*;
    use std::io::*;

    let file = File::create(filename).expect("can't make file!");
    let mut file = BufWriter::new(file); // BufWriter wrapper for performance

    for p in primes {
        let s = p.to_string() + "\n";
        file.write_all(s.as_bytes()).expect("can't write to file!");
    }
}
