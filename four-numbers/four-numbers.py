"""
Can two positive numbers be recovered from knowing their sum,
difference, product, and quotient, but in an unknown order?

We use the Z3 Theorem Prover to get the answer. To run, first
install Z3 via `pip3 install z3-solver` or by following the
instructions at:
    https://github.com/Z3Prover/z3

Problem statement:
For two positive numbers x and y, you are given four numbers
{a,b,c,d} and informed that they are the values of x+y, x-y, xy,
and x/y (in an unknown order). Show that this is enough
information to determine the values of both x and y.
"""

import itertools
import z3

s = z3.Solver()

# Consider two possible pairs x1, y1 and x2, y2

x1 = z3.Real("x1")
y1 = z3.Real("y1")
s.add(x1 > 0)
s.add(y1 > 0)

x2 = z3.Real("x2")
y2 = z3.Real("y2")
s.add(x2 > 0)
s.add(y2 > 0)

s.add(z3.Or(x1 != x2, y1 != y2))

# Make the bags {x+y, x-y, xy, x/y} and show
# that no permutation can be equal

l1 = (x1 + y1, x1 - y1, x1 * y1, x1 / y1)
l2 = (x2 + y2, x2 - y2, x2 * y2, x2 / y2)

results = {'sat': 0, 'unsat' : 0, 'unknown' : 0}
for p2 in itertools.permutations(l2):
    s.push()
    constraints = [l1[i] == p2[i] for i in range(4)]
    s.add(z3.And(*constraints))
    print(f"Solving constraints: {s.assertions()}")
    result = s.check()
    results[str(result)] += 1
    print(result)
    s.pop()

# Print Summary

print("===== Summary =====")
print(results)
