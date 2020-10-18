/*
    What is the smallest poset containing all posets of size n?

    For more details on the problem, see the header of `universal-poset.als`.
    This is an imperative solution in Rust, which aims to be more efficient
    so that we can get the answer for larger values of n.
*/

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

    #[derive(Clone, Debug)]
    pub struct Poset {
        pub size: Ele,
        pub edges: HashSet<(Ele, Ele)>,
    }
    impl Poset {
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
                    if edge1.1 == edge2.0 {
                        if !self.edges.contains(&(edge1.0, edge2.1)) {
                            return false;
                        }
                    }
                }
            }
            return true;
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
            for edge in &old_edges {
                if edge.1 == e1 {
                    self.add_edge_core(edge.0, e2);
                }
                if edge.0 == e2 {
                    self.add_edge_core(e1, edge.1);
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
    }
}

use poset::{Ele, Poset};

/*
    Enumerators for posets, up to isomorphism.

    These enumerators provide an at-least-once guarantee but
    do not guarantee that some posets will not be produced multiple times.
*/
// Enumerate all subsets of 0..size
fn enumerate_subsets(size: Ele) -> Vec<Vec<Ele>> {
    if size == 0 {
        return vec![Vec::new()];
    }
    let mut result = Vec::new();
    for mut subset in enumerate_subsets(size - 1).drain(..) {
        result.push(subset.clone());
        subset.push(size - 1);
        result.push(subset);
    }
    result
}
// Enumerate posets with size elements
fn enumerate_posets(size: Ele) -> Vec<Poset> {
    let mut result = Vec::new();
    // Base case
    if size == 0 {
        result.push(Poset::new_empty());
        return result;
    }
    // Strategy: enumerate posets of size size-1, then add a maximal element.
    let mut smaller_subsets = enumerate_subsets(size - 1);
    let smaller_posets = enumerate_posets(size - 1);
    for mut subset in smaller_subsets.drain(..) {
        for mut poset in smaller_posets.clone().drain(..) {
            poset.increase_size_by(1);
            for ele in subset.drain(..) {
                poset.add_edge(ele, size - 1);
            }
            result.push(poset);
        }
    }
    result
}
// Enumerate all candidates for a universal poset, using a few simple
// search optimizations.
fn enumerate_candidate_universal_posets(
    base_size: Ele, universal_size: Ele,
) -> Vec<Poset> {
    let min_size = if base_size == 0 { 0 } else { base_size * 2 - 1 };
    if universal_size < min_size {
        vec![]
    }
    else if universal_size == min_size {
        let mut universal = Poset::new_unordered(min_size);
        // 0 through base_size - 1 ordered
        // base_size through base_size * 2 - 2 unordered
        for i in 1..base_size {
            universal.add_edge(i - 1, i);
        }
        vec![universal]
    }
    else {
        let mut result = Vec::new();
        let smaller_posets = enumerate_candidate_universal_posets(
            base_size, universal_size - 1
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
                    if overlap { break; }
                }
                if overlap { continue; }
                for mut poset in smaller_posets.clone().drain(..) {
                    // If the subsets create a cycle, skip
                    let mut cycle = false;
                    for &ele1 in &subset1 {
                        for &ele2 in &subset2 {
                            if poset.edges.contains(&(ele2, ele1)) {
                                cycle = true;
                                break;
                            }
                        }
                        if cycle { break; }
                    }
                    if cycle { continue; }
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
// Enumerate all bijectionss from 0..size -> 0..size
fn enumerate_bijections(size: Ele) -> Vec<Vec<Ele>> {
    if size == 0 {
        return vec![vec![]];
    }
    let mut results = Vec::new();
    let smaller_bijections = enumerate_bijections(size - 1);
    for i in 0..size {
        // Element size-1 maps to i
        for mut bij in smaller_bijections.clone().drain(..) {
            assert_eq!(bij.len(), size - 1);
            for j in 0..(size - 1) {
                if bij[j] >= i {
                    bij[j] += 1;
                }
            }
            bij.push(i);
            results.push(bij);
        }
    }
    return results;
}

/*
    Solve the universal poset problem
*/
fn poset_contains(_p1: &Poset, _p2: &Poset) -> bool {
    // TODO: Implement
    true
}
fn solve_universal_poset(base_size: Ele) -> Ele {
    let base_posets = enumerate_posets(base_size);
    println!("Enumerated {} posets of size {}", base_posets.len(), base_size);
    for universal_size in base_size.. {
        let mut candidates = enumerate_candidate_universal_posets(
            base_size, universal_size
        );
        println!(
            "Testing {} candidates of size {}...",
            candidates.len(), universal_size
        );
        for candidate in candidates.drain(..) {
            let mut is_universal = true;
            for base_poset in &base_posets {
                if !poset_contains(base_poset, &candidate) {
                    is_universal = false;
                    break;
                }
            }
            if is_universal {
                println!("Universal poset found: {:?}", candidate);
                return universal_size;
            }
        }
    }
    unreachable!()
}

/*
    Entrypoint
*/
fn main() {
    println!("====== Universal Poset Problem Solution ======");
    let mut results = Vec::new();
    for n in 0..5 {
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

    const TEST_UPTO : Ele = 10;

    #[test]
    fn test_unordered_poset() {
        for n in 0..TEST_UPTO {
            let simple = Poset::new_unordered(n);
            assert_eq!(simple.size, n);
            assert_eq!(simple.edges.len(), n);
        }
    }
    #[test]
    fn test_line_poset() {
        for n in 1..TEST_UPTO {
            let mut line = Poset::new_unordered(n);
            for i in 1..n {
                line.add_edge(i - 1, i)
            }
            assert_eq!(line.size, n);
            assert_eq!(line.edges.len(), n * (n + 1) / 2)
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
        assert_eq!(enumerate_subsets(2), vec![
            vec![], vec![1], vec![0], vec![0, 1],
        ]);
        assert_eq!(enumerate_subsets(3), vec![
            vec![], vec![2], vec![1], vec![1, 2],
            vec![0], vec![0, 2], vec![0, 1], vec![0, 1, 2],
        ]);
    }

    #[test]
    fn test_enumerate_universal_posets_min_size() {
        for n in 0..TEST_UPTO {
            let min_size = if n == 0 { 0 } else { 2 * n - 1 };
            let num_edges = if n == 0 { 0 } else { n * (n + 1) / 2 + (n - 1) };
            let posets = enumerate_candidate_universal_posets(n, min_size);
            assert_eq!(posets.len(), 1);
            assert_eq!(posets[0].size, min_size);
            assert_eq!(posets[0].edges.len(), num_edges);
        }
    }
    #[test]
    fn test_enumerate_universal_posets_too_small() {
        for n in 1..TEST_UPTO {
            let too_small = 2 * n - 2;
            let posets = enumerate_candidate_universal_posets(n, too_small);
            assert_eq!(posets.len(), 0);
        }
    }

    #[test]
    fn test_enumerate_bijections() {
        assert_eq!(enumerate_bijections(0), vec![vec![],]);
        assert_eq!(enumerate_bijections(1), vec![vec![0],]);
        assert_eq!(enumerate_bijections(2), vec![vec![1, 0], vec![0, 1],]);
        assert_eq!(enumerate_bijections(3), vec![
            vec![2, 1, 0], vec![1, 2, 0],
            vec![2, 0, 1], vec![0, 2, 1],
            vec![1, 0, 2], vec![0, 1, 2],
        ]);
    }

    #[test]
    fn test_solve_universal_poset_easy() {
        assert_eq!(solve_universal_poset(0), 0);
        assert_eq!(solve_universal_poset(1), 1);
        assert_eq!(solve_universal_poset(2), 3);
        assert_eq!(solve_universal_poset(3), 5);
    }
    // Test doesn't work yet due to not implemented functionality
    // #[test]
    // fn test_solve_universal_poset_hard() {
    //     assert_eq!(solve_universal_poset(4), 8);
    // }
}
