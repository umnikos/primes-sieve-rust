use primes::*;
use quickcheck::*;
use quickcheck_macros::quickcheck;

#[test]
fn primes_to_7() {
    let res: Vec<usize> = make_primes(7).collect();
    assert_eq!(res, vec![2, 3, 5, 7]);
}

#[test]
fn primes_to_100() {
    let res: Vec<usize> = make_primes(100).collect();
    assert_eq!(
        res,
        vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97
        ]
    );
}

#[test]
fn primes_to_2() {
    let res: Vec<usize> = make_primes(2).collect();
    assert_eq!(res, vec![2]);
}

#[test]
fn primes_to_1() {
    let res: Vec<usize> = make_primes(1).collect();
    assert_eq!(res, vec![]);
}

#[test]
fn primes_to_0() {
    let res: Vec<usize> = make_primes(0).collect();
    assert_eq!(res, vec![]);
}

#[quickcheck]
fn prime_limit_not_undershot(limit: usize) -> TestResult {
    if !(2..=1_000_000).contains(&limit) {
        return TestResult::discard();
    }
    let primes: Vec<usize> = make_primes(limit).collect();
    TestResult::from_bool(
        make_primes(limit + 1000)
            .filter(|&p| p <= limit)
            .all(|p| primes.contains(&p)),
    )
}

#[quickcheck]
fn prime_limit_not_overshot(limit: usize) -> TestResult {
    if !(2..=1_000_000).contains(&limit) {
        return TestResult::discard();
    }
    TestResult::from_bool(make_primes(limit).all(|p| p <= limit))
}
