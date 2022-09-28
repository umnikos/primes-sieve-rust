use primes::*;

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
