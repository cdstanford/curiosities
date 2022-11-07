/*
    If you take a number and add the reverse of its digits in base 10,
    and then repeat this process, will this always eventually result in
    a palindrome?

    Examples:

    17 + 71 = 88

    49 + 94 = 143
    143 + 341 = 484
*/

type UINT = u128;

fn reverse(n: UINT) -> UINT {
    let rev_str: String = n.to_string().chars().rev().collect();
    rev_str.parse().unwrap()
}

#[test]
fn test_reverse() {
    assert_eq!(reverse(0), 0);
    assert_eq!(reverse(7), 7);
    assert_eq!(reverse(10), 1);
    assert_eq!(reverse(372), 273);
    assert_eq!(reverse(101), 101);
    assert_eq!(reverse(47000), 74);
}

fn is_palindrome(n: UINT) -> bool {
    n == reverse(n)
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

fn step(n: UINT) -> UINT {
    n + reverse(n)
}

#[test]
fn test_step() {
    assert_eq!(step(1), 2);
    assert_eq!(step(17), 88);
    assert_eq!(step(49), 143);
    assert_eq!(step(143), 484);
}

const MAX_STEPS: UINT = 1000;

fn steps(mut n: UINT) -> Option<UINT> {
    let mut count = 0;
    while !is_palindrome(n) {
        n = step(n);
        count += 1;
        if count >= MAX_STEPS {
            return None;
        }
    }
    Some(count)
}

#[test]
fn test_steps() {
    assert_eq!(steps(11), Some(0));
    assert_eq!(steps(17), Some(1));
    assert_eq!(steps(49), Some(2));
}

fn display_steps(n: UINT) -> String {
    match steps(n) {
        Some(m) => format!("{n}: {m} steps"),
        None => format!("{n}: not found (tried {MAX_STEPS})"),
    }
}

fn main() {
    println!("Palindrome test");
    for n in 0.. {
        println!("{}", display_steps(n));
    }
}
