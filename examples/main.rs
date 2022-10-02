use inquire;
use inquire::validator::Validation;
use primes::make_primes;

fn main() {
    let limit: usize = prompt_user_for_limit();
    let primes = make_primes(limit);
    write_to_file(primes);
    println!("done!");
}

fn prompt_user_for_limit() -> usize {
    inquire::CustomType::<usize>::new("Generate primes up to:")
        .with_validator(|&input: &usize| {
            if input >= 2 {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Limit should be at least 2".into()))
            }
        })
        .prompt()
        .unwrap()
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
