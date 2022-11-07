/*
    Problem Statement:
    If you take a number and add the reverse of its digits in base 10,
    and then repeat this process, will this always eventually result in
    a palindrome?

    Examples:

    17 + 71 = 88

    49 + 94 = 143
    143 + 341 = 484
*/

type Uint = u128;

fn reverse(n: Uint) -> Option<Uint> {
    // return None on overflow
    let rev_str: String = n.to_string().chars().rev().collect();
    rev_str.parse().ok()
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(0), Some(0));
    assert_eq!(reverse(7), Some(7));
    assert_eq!(reverse(10), Some(1));
    assert_eq!(reverse(372), Some(273));
    assert_eq!(reverse(101), Some(101));
    assert_eq!(reverse(47000), Some(74));
}

fn is_palindrome(n: Uint) -> bool {
    reverse(n) == Some(n)
}

#[test]
fn test_palindrome() {
    assert!(is_palindrome(0));
    assert!(is_palindrome(5));
    assert!(is_palindrome(33));
    assert!(!is_palindrome(34));
    assert!(!is_palindrome(50));
    assert!(is_palindrome(1056501));
    assert!(is_palindrome(105501));
    assert!(!is_palindrome(105601));
}

fn step(n: Uint) -> Option<Uint> {
    // return None on overflow
    n.checked_add(reverse(n)?)
}

#[test]
fn test_step() {
    assert_eq!(step(1), Some(2));
    assert_eq!(step(17), Some(88));
    assert_eq!(step(49), Some(143));
    assert_eq!(step(143), Some(484));
}

const MAX_STEPS: Uint = 1000;

fn steps(n: Uint) -> Option<Uint> {
    let mut count = 0;
    let mut n_mut = n;
    while !is_palindrome(n_mut) {
        count += 1;
        match step(n_mut) {
            Some(n_new) => {
                n_mut = n_new;
            }
            None => {
                println!("integer overflow: {} -> ? (tried {} steps)", n, count);
                return None;
            }
        }
        if count >= MAX_STEPS {
            println!("not found: {} -> ? (tried {} steps)", n, count);
            return None;
        }
    }
    println!("palindrome found: {} -> {} ({} steps)", n, n_mut, count);
    Some(count)
}

#[test]
fn test_steps() {
    assert_eq!(steps(11), Some(0));
    assert_eq!(steps(17), Some(1));
    assert_eq!(steps(49), Some(2));
}

const RESULTS_UPTO: usize = 100;

fn main() {
    println!("=== Palindrome test ===");
    let mut results: Vec<Vec<Uint>> = vec![Vec::new(); RESULTS_UPTO];
    let mut results_many = Vec::new();
    let mut results_notfound = Vec::new();
    for n in 0..1000 {
        match steps(n) {
            Some(i) => {
                let i = i as usize;
                if i < RESULTS_UPTO {
                    results[i].push(n);
                } else {
                    results_many.push(n);
                }
            }
            None => results_notfound.push(n),
        }
    }
    println!("=== Results summary ===");
    for (i, results_i) in results.iter().enumerate() {
        if !results_i.is_empty() {
            println!("Reached a palindrome in {} steps: {:?}", i, results_i);
        }
    }
    println!(
        "Reached a palindrome in {}+ steps: {:?}",
        RESULTS_UPTO, results_many
    );
    println!("Palindrome not found: {:?}", results_notfound);
}
