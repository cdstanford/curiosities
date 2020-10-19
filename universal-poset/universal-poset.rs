/*
    What is the smallest poset containing all posets of size n?

    For more details on the problem, see the header of `universal-poset.als`.
    This is an imperative solution in Rust, which aims to be more efficient
    so that we can get the answer for larger values of n.
*/

use std::cmp::Ordering;
use std::vec::Vec;

/*
    Struct for a finite (explicitly enumerated) poset.

    As posets are expected to be quite small for this problem,
    this is naively implemented using a single HashSet for the entire
    set of edges, and using an edge for every ordering u <= v rather
    than just for the primitive edges u <= v (primitive meaning that there
    is no w such that u <= w <= v).
*/

mod poset {
    use std::collections::HashSet;
    pub type Ele = usize;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Poset {
        size: Ele,
        edges: HashSet<(Ele, Ele)>,
    }
    impl Poset {
        /* Getters */
        pub fn get_size(&self) -> Ele {
            self.size
        }
        pub fn get_num_edges(&self) -> Ele {
            self.edges.len()
        }
        pub fn contains_edge(&self, u: Ele, v: Ele) -> bool {
            self.edges.contains(&(u, v))
        }
        /* Object Invariant */
        fn invariant(&self) -> bool {
            // Check elements are in range
            for edge in &self.edges {
                if edge.0 >= self.size || edge.1 >= self.size {
                    return false;
                }
            }
            // Check edges satisfy reflexivity
            for i in 0..self.size {
                if !self.edges.contains(&(i, i)) {
                    return false;
                }
            }
            // Check edges satisfy transitivity
            for edge1 in &self.edges {
                for edge2 in &self.edges {
                    if edge1.1 == edge2.0
                        && !self.edges.contains(&(edge1.0, edge2.1))
                    {
                        return false;
                    }
                }
            }
            true
        }
        fn assert_invariant(&self) {
            // No-op in release mode
            debug_assert!(self.invariant());
        }

        /* Constructor */
        pub fn new_unordered(size: Ele) -> Self {
            let mut edges = HashSet::new();
            for e in 0..size {
                edges.insert((e, e));
            }
            let result = Self { size, edges };
            result.assert_invariant();
            result
        }
        pub fn new_empty() -> Self {
            Self::new_unordered(0)
        }

        /* Primitive modifiers: tehse do NOT preserve the invariant */
        fn increase_size_by_core(&mut self, size: Ele) {
            self.size += size;
        }
        fn add_edge_core(&mut self, e1: Ele, e2: Ele) {
            self.edges.insert((e1, e2));
        }

        /* High-level operations */
        // Add element(s) and enforce reflexivity
        pub fn increase_size_by(&mut self, size: Ele) {
            for e in self.size..(self.size + size) {
                self.add_edge_core(e, e);
            }
            self.increase_size_by_core(size);
            self.assert_invariant();
        }
        // Add an ordering and enforce transitivity
        pub fn add_edge(&mut self, e1: Ele, e2: Ele) {
            assert!(e1 != e2 && e1 < self.size && e2 < self.size);
            assert!(!self.edges.contains(&(e2, e1)));
            let old_edges = self.edges.clone();
            self.add_edge_core(e1, e2);
            for edge1 in &old_edges {
                if edge1.1 == e1 {
                    for edge2 in &old_edges {
                        if edge2.0 == e2 {
                            self.add_edge_core(edge1.0, edge2.1);
                        }
                    }
                }
            }
            self.assert_invariant();
        }
        // Disjoint union of two posets
        #[allow(dead_code)]
        pub fn union(&mut self, other: &Self) {
            self.increase_size_by_core(other.size);
            for (e1, e2) in &other.edges {
                self.add_edge_core(self.size + e1, self.size + e2);
            }
            self.assert_invariant();
        }
        // Check if one poset contains another
        pub fn embeds_in(&self, other: &Self) -> bool {
            use crate::enumerate_injections;
            let mut injections = enumerate_injections(self.size, other.size);
            for inj in injections.drain(..) {
                let mut skip = false;
                for e1 in 0..(self.size) {
                    for f1 in 0..(self.size) {
                        let e2 = inj[e1];
                        let f2 = inj[f1];
                        if self.edges.contains(&(e1, f1))
                            != other.edges.contains(&(e2, f2))
                        {
                            skip = true;
                            break;
                        }
                    }
                    if skip {
                        break;
                    }
                }
                if !skip {
                    // This injection works
                    return true;
                }
            }
            false
        }
        // Check if two posets are isomorphic
        pub fn isomorphic(&self, other: &Self) -> bool {
            self.size == other.size
                && self.edges.len() == other.edges.len()
                && self.embeds_in(other)
        }
    }
}

use poset::{Ele, Poset};

/*
    Some useful utility enumerators, before we enumerate posets
*/

// Enumerate all subsets of 0..size
fn enumerate_subsets(size: Ele) -> Vec<Vec<Ele>> {
    if size == 0 {
        vec![Vec::new()]
    } else {
        let mut results = Vec::new();
        for mut subset in enumerate_subsets(size - 1).drain(..) {
            results.push(subset.clone());
            subset.push(size - 1);
            results.push(subset);
        }
        results
    }
}

// Enumerate all partitions of size into parts parts
fn enumerate_partitions(size: Ele) -> Vec<Vec<Ele>> {
    if size == 0 {
        vec![Vec::new()]
    } else {
        let mut results = Vec::new();
        for last_ele in 1..(size + 1) {
            for mut subset in enumerate_partitions(size - last_ele).drain(..) {
                subset.push(last_ele);
                results.push(subset);
            }
        }
        results
    }
}

// Enumerate all injections from 0..isize -> 0..osize
fn enumerate_injections(isize: Ele, osize: Ele) -> Vec<Vec<Ele>> {
    if isize == 0 {
        return vec![vec![]];
    } else if osize == 0 {
        return vec![];
    }
    let mut results = Vec::new();
    let smaller_injections = enumerate_injections(isize - 1, osize - 1);
    for i in 0..osize {
        // Element isize-1 maps to i
        for mut inj in smaller_injections.clone().drain(..) {
            assert_eq!(inj.len(), isize - 1);
            for item in inj.iter_mut() {
                if *item >= i {
                    *item += 1;
                }
            }
            inj.push(i);
            results.push(inj);
        }
    }
    results
}
// Enumerate all bijections from 0..size -> 0..size
#[allow(dead_code)]
fn enumerate_bijections(size: Ele) -> Vec<Vec<Ele>> {
    enumerate_injections(size, size)
}

/*
    Enumerators for posets, up to isomorphism.

    Some of these enumerators provide an at-least-once guarantee but
    do not guarantee that some posets will not be produced multiple times.
    Others are exactly-once.
*/

// Enumerate posets given a list of the number of elements at each level
// Each element at level 1 must be greater than at least one element at level 0,
// and so on.
// Precondition:
//   - each element of level_sizes is nonzero, except possibly the last one
//   - the sum of these elements is total_size
// Guarantee: at-least-once
fn enumerate_posets_leveled_rec(
    level_sizes: &mut Vec<Ele>,
    total_size: Ele,
) -> Vec<Poset> {
    // println!("enumerate_posets_leveled_rec: {:?}, {}", level_sizes, total_size);
    let numlevels = level_sizes.len();
    if numlevels == 0 {
        vec![Poset::new_empty()]
    } else if level_sizes[numlevels - 1] == 0 {
        level_sizes.pop();
        let results = enumerate_posets_leveled_rec(level_sizes, total_size);
        level_sizes.push(0);
        results
    } else if numlevels == 1 {
        debug_assert!(level_sizes[0] == total_size);
        vec![Poset::new_unordered(total_size)]
    } else {
        // Make a list of all subsets of the previous levels that have at
        // least one element in the top level
        let prev_size = total_size - level_sizes[numlevels - 1];
        let prevprev_size = prev_size - level_sizes[numlevels - 2];
        let mut subsets = enumerate_subsets(prev_size);
        let mut good_subsets = Vec::new();
        for subset in subsets.drain(..) {
            if !subset.is_empty() && subset[subset.len() - 1] >= prevprev_size {
                good_subsets.push(subset);
            }
        }
        // Recurse to find all smaller posets
        level_sizes[numlevels - 1] -= 1;
        let mut subposets =
            enumerate_posets_leveled_rec(level_sizes, total_size - 1);
        level_sizes[numlevels - 1] += 1;
        // For all prev posets, add elements dependent on the good subsets
        let mut results = Vec::new();
        for mut poset in subposets.drain(..) {
            poset.increase_size_by(1);
            for subset in &good_subsets {
                // println!("  poset: {:?}", poset);
                // println!("  subset: {:?}", subset);
                let mut result = poset.clone();
                for &prev_ele in subset {
                    // println!("  adding edge: {}, {}", prev_ele, total_size - 1);
                    result.add_edge(prev_ele, total_size - 1);
                }
                // println!("  result: {:?}", result);
                results.push(result);
            }
        }
        results
    }
}
// Unlike the _rec version, this one provides an exactly-once guarantee
fn enumerate_posets_leveled(level_sizes: &mut Vec<Ele>) -> Vec<Poset> {
    // println!("enumerate_posets_leveled: {:?}", level_sizes);
    let mut results = Vec::new();
    let posets =
        enumerate_posets_leveled_rec(level_sizes, level_sizes.iter().sum());
    for (i, poset_ref) in posets.iter().enumerate() {
        // Only add if not isomorphic to any earlier poset
        let mut isomorphic = false;
        for other_ref in posets.iter().take(i) {
            if poset_ref.isomorphic(other_ref) {
                isomorphic = true;
                break;
            }
        }
        if !isomorphic {
            results.push(poset_ref.clone());
        }
    }
    results
}

// Enumerate posets with size elements.
// Guarantee: exactly-once
fn enumerate_posets(size: Ele) -> Vec<Poset> {
    // println!("enumerate_posets: {:?}", size);
    if size == 0 {
        vec![Poset::new_empty()]
    } else {
        let mut results = Vec::new();
        for mut partition in enumerate_partitions(size).drain(..) {
            results.append(&mut enumerate_posets_leveled(&mut partition));
        }
        results
    }
    // // Old strategy: enumerate posets of size size-1, then add a maximal element.
    // let mut smaller_subsets = enumerate_subsets(size - 1);
    // let smaller_posets = enumerate_posets(size - 1);
    // for mut subset in smaller_subsets.drain(..) {
    //     for mut poset in smaller_posets.clone().drain(..) {
    //         poset.increase_size_by(1);
    //         for ele in subset.drain(..) {
    //             poset.add_edge(ele, size - 1);
    //         }
    //         result.push(poset);
    //     }
    // }
    // result
}
// Enumerate all candidates for a universal poset, using a few simple
// search optimizations.
// This is a super gnarly function and should be rewritten...
fn enumerate_candidate_universal_posets(
    base_size: Ele,
    universal_size: Ele,
) -> Vec<Poset> {
    // let min_size = if base_size == 0 { 0 } else { base_size * 2 - 1 };
    let min_size = base_size;
    match universal_size.cmp(&min_size) {
        Ordering::Less => vec![],
        Ordering::Equal => {
            let mut universal = Poset::new_unordered(min_size);
            // 0 through base_size - 1 ordered
            // base_size through base_size * 2 - 2 unordered
            for i in 1..base_size {
                universal.add_edge(i - 1, i);
            }
            vec![universal]
        }
        Ordering::Greater => {
            let mut result = Vec::new();
            let smaller_posets = enumerate_candidate_universal_posets(
                base_size,
                universal_size - 1,
            );
            let smaller_subsets = enumerate_subsets(universal_size - 1);
            for subset1 in smaller_subsets.clone().drain(..) {
                for subset2 in smaller_subsets.clone().drain(..) {
                    // If the subsets overlap, skip
                    let mut overlap = false;
                    for &ele1 in &subset1 {
                        for &ele2 in &subset2 {
                            if ele1 == ele2 {
                                overlap = true;
                                break;
                            }
                        }
                        if overlap {
                            break;
                        }
                    }
                    if overlap {
                        continue;
                    }
                    for mut poset in smaller_posets.clone().drain(..) {
                        // If the subsets create a cycle, skip
                        let mut cycle = false;
                        for &ele1 in &subset1 {
                            for &ele2 in &subset2 {
                                if poset.contains_edge(ele2, ele1) {
                                    cycle = true;
                                    break;
                                }
                            }
                            if cycle {
                                break;
                            }
                        }
                        if cycle {
                            continue;
                        }
                        poset.increase_size_by(1);
                        for ele in subset1.clone().drain(..) {
                            poset.add_edge(ele, universal_size - 1);
                        }
                        for ele in subset2.clone().drain(..) {
                            poset.add_edge(universal_size - 1, ele);
                        }
                        result.push(poset);
                    }
                }
            }
            result
        }
    }
}

/*
    Solve the universal poset problem
*/
fn solve_universal_poset(base_size: Ele) -> Ele {
    let base_posets = enumerate_posets(base_size);
    // println!("Enumerated {} posets of size {}", base_posets.len(), base_size);
    for universal_size in base_size.. {
        let mut candidates =
            enumerate_candidate_universal_posets(base_size, universal_size);
        let num_candidates = candidates.len();
        for (num, candidate) in candidates.drain(..).enumerate() {
            print!(
                "\rTesting {} candidates of size {}... ({} complete)",
                num_candidates, universal_size, num
            );
            let mut is_universal = true;
            for base_poset in &base_posets {
                if !base_poset.embeds_in(&candidate) {
                    is_universal = false;
                    break;
                }
            }
            print!(
                "\rTesting {} candidates of size {}... ({} complete)",
                num_candidates,
                universal_size,
                num + 1
            );
            if is_universal {
                println!("\nUniversal poset found: {:?}", candidate);
                return universal_size;
            }
        }
        println!();
    }
    unreachable!()
}

/*
    Entrypoint
*/
fn main() {
    println!("====== Number of Posets ======");
    for n in 0..7 {
        let posets = enumerate_posets(n);
        println!("{}: {}", n, posets.len());
    }
    println!("====== Universal Poset Problem Solution ======");
    let mut results = Vec::new();
    for n in 0..10 {
        println!("==== n = {} ====", n);
        results.push(solve_universal_poset(n));
    }
    println!("==== Summary ====");
    for (n, ans) in results.iter().enumerate() {
        println!("{}: {}", n, ans);
    }
}

/*
    Unit Tests
*/
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const TEST_UPTO_BIG: Ele = 10;
    const TEST_UPTO_SMALL: Ele = 5;

    #[test]
    fn test_unordered_poset() {
        for n in 0..TEST_UPTO_BIG {
            let simple = Poset::new_unordered(n);
            assert_eq!(simple.get_size(), n);
            assert_eq!(simple.get_num_edges(), n);
        }
    }
    #[test]
    fn test_line_poset() {
        for n in 1..TEST_UPTO_BIG {
            let mut line = Poset::new_unordered(n);
            for i in 1..n {
                line.add_edge(i - 1, i)
            }
            assert_eq!(line.get_size(), n);
            assert_eq!(line.get_num_edges(), n * (n + 1) / 2)
        }
    }
    #[test]
    #[should_panic]
    fn test_toosmall() {
        let mut poset = Poset::new_unordered(2);
        poset.add_edge(1, 2);
    }
    #[test]
    #[should_panic]
    fn test_cycle() {
        let mut poset = Poset::new_unordered(3);
        poset.add_edge(0, 1);
        poset.add_edge(1, 2);
        poset.add_edge(2, 0);
    }

    #[test]
    fn test_enumerate_subsets() {
        assert_eq!(enumerate_subsets(0), vec![vec![],]);
        assert_eq!(enumerate_subsets(1), vec![vec![], vec![0],]);
        assert_eq!(
            enumerate_subsets(2),
            vec![vec![], vec![1], vec![0], vec![0, 1],]
        );
        assert_eq!(
            enumerate_subsets(3),
            vec![
                vec![],
                vec![2],
                vec![1],
                vec![1, 2],
                vec![0],
                vec![0, 2],
                vec![0, 1],
                vec![0, 1, 2],
            ]
        );
    }

    #[test]
    fn test_enumerate_partitions() {
        assert_eq!(enumerate_partitions(0), vec![vec![],]);
        assert_eq!(enumerate_partitions(1), vec![vec![1],]);
        assert_eq!(enumerate_partitions(2), vec![vec![1, 1], vec![2],]);
        assert_eq!(
            enumerate_partitions(3),
            vec![vec![1, 1, 1], vec![2, 1], vec![1, 2], vec![3],]
        );
    }

    #[test]
    fn test_enumerate_bijections() {
        assert_eq!(enumerate_bijections(0), vec![vec![],]);
        assert_eq!(enumerate_bijections(1), vec![vec![0],]);
        assert_eq!(enumerate_bijections(2), vec![vec![1, 0], vec![0, 1],]);
        assert_eq!(
            enumerate_bijections(3),
            vec![
                vec![2, 1, 0],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![0, 1, 2],
            ]
        );
    }

    #[test]
    fn test_enumerate_injections() {
        assert_eq!(enumerate_injections(0, 1), vec![vec![],]);
        assert_eq!(enumerate_injections(1, 2), vec![vec![0], vec![1],]);
        assert_eq!(enumerate_injections(2, 1), vec![] as Vec<Vec<Ele>>);
        assert_eq!(
            enumerate_injections(2, 3),
            vec![
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![2, 1],
                vec![0, 2],
                vec![1, 2],
            ]
        );
    }

    #[test]
    fn test_enumerate_posets_leveled() {
        assert_eq!(
            enumerate_posets_leveled(&mut vec![]),
            vec![Poset::new_empty()]
        );
        for n in 1..TEST_UPTO_SMALL {
            assert_eq!(
                enumerate_posets_leveled(&mut vec![n]),
                vec![Poset::new_unordered(n)]
            );
            assert_eq!(enumerate_posets_leveled(&mut vec![n, 1]).len(), n);
            assert_eq!(enumerate_posets_leveled(&mut vec![1, n]).len(), 1);
            assert_eq!(enumerate_posets_leveled(&mut vec![1, n, 1]).len(), n);
            assert_eq!(
                enumerate_posets_leveled(&mut vec![n, 1, 1]).len(),
                ((n + 2) * (n + 1) / 2) - (n + 1)
            );
            // Disabled slow test
            // assert_eq!(
            //     enumerate_posets_leveled(&mut vec![1, n, n]).len(),
            //     enumerate_posets_leveled(&mut vec![n, n]).len()
            // );
        }
        assert_eq!(enumerate_posets_leveled(&mut vec![2, 2]).len(), 4);
        assert_eq!(enumerate_posets_leveled(&mut vec![2, 3]).len(), 6);
        assert_eq!(enumerate_posets_leveled(&mut vec![3, 2]).len(), 9);
    }

    #[test]
    fn test_enumerate_posets() {
        // https://oeis.org/A000112
        assert_eq!(enumerate_posets(0).len(), 1);
        assert_eq!(enumerate_posets(1).len(), 1);
        assert_eq!(enumerate_posets(2).len(), 2);
        assert_eq!(enumerate_posets(3).len(), 5);
        assert_eq!(enumerate_posets(4).len(), 16);
        assert_eq!(enumerate_posets(5).len(), 63);
        // Disabled slow test
        // assert_eq!(enumerate_posets(6).len(), 318);
    }

    #[test]
    fn test_enumerate_universal_posets_min_size() {
        for n in 0..TEST_UPTO_BIG {
            // Old way of enumerating candidate posets
            // let min_size = if n == 0 { 0 } else { 2 * n - 1 };
            let min_size = n;
            let num_edges = n * (n + 1) / 2;
            let posets = enumerate_candidate_universal_posets(n, min_size);
            assert_eq!(posets.len(), 1);
            assert_eq!(posets[0].get_size(), min_size);
            assert_eq!(posets[0].get_num_edges(), num_edges);
        }
    }
    #[test]
    fn test_enumerate_universal_posets_too_small() {
        for n in 1..TEST_UPTO_BIG {
            // Old way of enumerating candidate posets
            // let too_small = 2 * n - 2;
            let too_small = n - 1;
            let posets = enumerate_candidate_universal_posets(n, too_small);
            assert_eq!(posets.len(), 0);
        }
    }

    #[test]
    fn test_solve_universal_poset_easy() {
        assert_eq!(solve_universal_poset(0), 0);
        assert_eq!(solve_universal_poset(1), 1);
        assert_eq!(solve_universal_poset(2), 3);
        assert_eq!(solve_universal_poset(3), 5);
    }
    // Disabled slow test
    // #[test]
    // fn test_solve_universal_poset_hard() {
    //     assert_eq!(solve_universal_poset(4), 8);
    // }
}