"""
Problem statement:
    You are given a deck of 30 cards which have the distinct real
    numbers a, b, or c written on them: ten with a, ten with b, and
    ten with c. You are told that, for any set of five cards from
    this deck, it is possible to choose another five cards such that
    the total sum of all ten cards is zero. Prove that one of a, b,
    or c must be zero.

This is a solution using the Z3 Theorem Prover. To run, first install
Z3 via `pip3 install z3-solver` or by following the instructions at:
    https://github.com/Z3Prover/z3

Output of `time python3 three-reals.py`:
    ===== Solution with any a, b, c =====
    Result: sat
    Model: [b = 0, a = 1/16, c = -1/16]
    ===== Solution with a, b, c nonzero =====
    Result: unsat

    real    0m11.207s
    user    0m11.175s
    sys     0m0.032s
"""

import itertools
import z3

def print_solution(solver):
    result = str(solver.check())
    print(f"Result: {result}")
    if result == 'sat':
        print(f"Model: {solver.model()}")

solver = z3.Solver()

# Distinct real numbers a, b, c
a = z3.Real("a")
b = z3.Real("b")
c = z3.Real("c")
solver.add(a != b)
solver.add(a != c)
solver.add(b != c)

# For all possible hands of 5 cards, there exists another hand such
# that the sum of the two hands is zero
hands = list(itertools.combinations_with_replacement((a, b, c), 5))
for hand1 in hands:
    possible_sums = [sum(hand1) + sum(hand2) for hand2 in hands]
    constraint = z3.Or(*[pos_sum == 0 for pos_sum in possible_sums])
    print(f"Adding constraint: {constraint}")
    solver.add(constraint)

print(f"===== Solution with any a, b, c =====")
print_solution(solver) # sat

print(f"===== Solution with a, b, c nonzero =====")
solver.add(a != 0)
solver.add(b != 0)
solver.add(c != 0)
print_solution(solver) # unsat
