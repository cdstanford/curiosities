/*
    What is the smallest poset containing all posets of size n?

    This code uses the Alloy language (https://alloytools.org/) to get the
    answer by encoding the problem as a SAT instance.

    Spoiler: the encoding is expensive, and only tractable for n <= 4.
    If f(n) is the minimum size of the universal poset for the class of
    posets of size n, then we are able to verify:
    - f(1) = 1
    - f(2) = 3
    - f(3) = 5
    - f(4) = 8

    References:
    - The problem is considered in general here, where it is shown that there
      is no polynomial upper bound:
      https://mathoverflow.net/questions/25874/
    - Currently the sequence doesn't seem to be in OEIS.
    - In the code below, we also need to hardcode the number of posets on
      n labeled elements, which can be found here:
      https://oeis.org/A001035
*/

// First, we model the problem with an abstract set of n vertices V, and a
// (larger) set of vertices UniversalV.

sig UniversalV {}
one sig UniversalPoset {
    edge: UniversalV -> UniversalV,
}{
    // edge is reflexive and transitive
    edge = (UniversalV <: *edge)
    // edge is antisymmetric
    edge & ~edge in iden
}

sig V {}
sig Poset {
    edge: V -> V,
    embedding: V -> one UniversalV,
}{
    // edge is reflexive and transitive
    edge = (V <: *edge)
    // edge is antisymmetric
    edge & ~edge in iden
}

// The poset relation on UniversalV should contain all possible posets on V.

pred poset_embeds[p: Poset] {
    p.edge = (p.embedding).(UniversalPoset.edge).(~(p.embedding))
}

fact universal_poset_is_universal {
    all p: Poset | poset_embeds[p]
}

fact different_posets_are_different {
    all p1: Poset | all p2: Poset {
        (p1.edge = p2.edge) => p1 = p2
    }
}

// Finally, we verify the solution for different values of n.
// For each n, we check that the minimum size is satisfiable, then check that
// (a) one less than the minimum is unsatisfiable; and
// (b) one additional poset is unsatisfiable.

/* n = 1 */
// Number of posets: 1
// Minimum universal poset size: 1
run sat_1 {} for exactly 1 V, exactly 1 Poset, exactly 1 UniversalV
run unsat_1a {} for exactly 1 V, exactly 1 Poset, exactly 0 UniversalV
run unsat_1b {} for exactly 1 V, exactly 2 Poset, exactly 1 UniversalV

/* n = 2 */
// Number of posets: 3
// Minimum universal poset size: 3
run sat_2 {} for exactly 2 V, exactly 3 Poset, exactly 3 UniversalV
run unsat_2a {} for exactly 2 V, exactly 3 Poset, exactly 2 UniversalV
run unsat_2b {} for exactly 2 V, exactly 4 Poset, exactly 3 UniversalV

/* n = 3 */
// Number of posets: 19
// Minimum universal poset size: 5
run sat_3 {} for exactly 3 V, exactly 19 Poset, exactly 5 UniversalV
run unsat_3a {} for exactly 3 V, exactly 19 Poset, exactly 4 UniversalV
run unsat_3b {} for exactly 3 V, exactly 20 Poset, exactly 5 UniversalV

/* n = 4 */
// Number of posets: 219
// Minimum universal poset size: 8
// Note: this runs slowly (a few minutes per query).
run sat_4 {} for exactly 4 V, exactly 219 Poset, exactly 8 UniversalV
run unsat_4a {} for exactly 4 V, exactly 219 Poset, exactly 7 UniversalV
run unsat_4b {} for exactly 4 V, exactly 220 Poset, exactly 8 UniversalV
