/*
    For fun, some code in Rust which checks the Union closed sets conjecture:
    https://en.wikipedia.org/wiki/Union-closed_sets_conjecture
*/

use std::collections::HashSet;
use std::ops::Add;

/*
    Represent a finite set of nonnegative integers as a single nonnegative integer
    via binary encoding, e.g.:
        {} = 0
        {0} = 1
        {1} = 10
        {0, 1} = 11

    Note that all nonnegative integers represent valid sets this way.
*/
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntSet(u128);
impl IntSet {
    pub fn new(v: Vec<u8>) -> Self {
        let mut result: u128 = 0;
        for x in v {
            debug_assert!(x <= 127);
            result |= 2_u128.checked_pow(x as u32).expect("int too large");
        }
        Self(result)
    }
}
#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for &IntSet {
    type Output = IntSet;
    fn add(self, other: Self) -> Self::Output {
        IntSet(self.0 | other.0)
    }
}

pub fn close_unions(sets: &mut HashSet<IntSet>) {
    sets.insert(IntSet(1));
    let original = sets.clone();
    let mut frontier: Vec<IntSet> = sets.iter().cloned().collect();
    while !frontier.is_empty() {
        let set1 = &frontier.pop().unwrap();
        for set2 in &original {
            let set = set1 + set2;
            if sets.insert(set) {
                frontier.push(set);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let emp = IntSet::new(vec![]);
        let s0 = IntSet::new(vec![0]);
        let s1 = IntSet::new(vec![1]);
        let s01 = IntSet::new(vec![0, 1]);
        let s12 = IntSet::new(vec![1, 2]);
        let s14 = IntSet::new(vec![1, 4]);
        let s014 = IntSet::new(vec![0, 1, 4]);

        assert_eq!(&emp + &s0, s0);
        assert_eq!(&s0 + &s0, s0);
        assert_eq!(&s0 + &s1, s01);
        assert_eq!(&s12 + &emp, s12);
        assert_eq!(&s12 + &s12, s12);
        assert_eq!(&s0 + &s14, s014);
        assert_eq!(&s01 + &s14, s014);
    }

    // #[test]
    // fn test_close_unions {
    //     let s = IntSet::new(vec![]);
    //     close_unions(&mut s);
    //     assert_eq!()
    //     let s0 = IntSet::new(vec![0]);
    //     let s1 = IntSet::new(vec![1]);
    //     let s01 = IntSet::new(vec![0, 1]);
    //     let s12 = IntSet::new(vec![1, 2]);
    //     let s14 = IntSet::new(vec![1, 4]);
    //     let s014 = IntSet::new(vec![0, 1, 4]);
    // }
}
