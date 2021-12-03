"""
Can 7 points be arranged on the plane such that, for any choice of 3
points among them, 2 of the points are unit-distance apart?

This solution uses the Z3 Theorem Prover:
    https://github.com/Z3Prover/z3

Z3 currently finds a solution for up to 6 points, but hangs for 7 points.
"""

import z3

def point(i):
    return (z3.Real(f"x{i}"), z3.Real(f"y{i}"))

def is_valid(p):
    return z3.And(p[0] > 0, p[1] > 0)

def unit_distance(p, q):
    dx = q[0] - p[0]
    dy = q[1] - p[1]
    return dx * dx + dy * dy == 1

def solve_points(n):
    # Solve the problem for n points.
    s = z3.Solver()
    points = [point(i) for i in range(n)]
    for i in range(n):
        s.add(is_valid(points[i]))
    for i in range(n):
        for j in range(i + 1, n):
            for k in range(j + 1, n):
                s.add(z3.Or(
                    unit_distance(points[i], points[j]),
                    unit_distance(points[i], points[k]),
                    unit_distance(points[j], points[k]),
                ))
    result = s.check()
    print(f"===== Solution with {n} points =====")
    if str(result) == "sat":
        print(f"sat: {s.model()}")
    else:
        print(result)

for n in range(8):
    solve_points(n)
