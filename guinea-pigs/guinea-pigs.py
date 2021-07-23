"""
This problem is from the CAVIA quiz held at the virtual conference CAV 2021 on
Thursday, July 22.

Anita and Bertram are two guinea pigs using the latest guinea pig technology:
the guinea pig bridge (length: 2 meters). Bertram starts crossing first,
at a speed of 0.2 meters per second. One second later, Anita starts crossing
the bridge as well, at a speed of 0.3 meters per second.Anita is a polite
guinea pig, so when she bumps into Bertram, she waits until he has reached the
end of the bridge before starting to move again, at her original speed.

When does Anita finish crossing the guinea pig bridge? You may assume the
guinea pigs to be a point mass (no length between snout and tail).
"""

import z3

def print_solution(solver):
    result = str(solver.check())
    print(f"Result: {result}")
    if result == 'sat':
        print(f"Model: {solver.model()}")

solver = z3.Solver()

t1 = z3.Real("t1")
t2 = z3.Real("t2")
t3 = z3.Real("t3")
t4 = z3.Real("t4")
total = z3.Real("total")

d1 = z3.Real("d1")
d2 = z3.Real("d2")

solver.add(t1 == 1)
solver.add(t1 * .2 == d1)
solver.add((t1 + t2) * .2 == t2 * .3)
solver.add((t1 + t2 + t3) * .2 == 2)
solver.add((t2 + t4) * .3 == 2)
solver.add(t1 + t2 + t3 + t4 == total)

# Optional additional constraints
solver.add(t1 > 0)
solver.add(t2 > 0)
solver.add(t3 > 0)
solver.add(t4 > 0)
solver.add(d1 > 0)
solver.add(d2 > 0)
solver.add(d1 + d2 < 2)

print_solution(solver)
