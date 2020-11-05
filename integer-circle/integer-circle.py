"""
The integers 1 through 100 are written on a circle in an arbitrary order.
Is it possible for the absolute difference between every two adjacent integers
to be between 30 and 50 inclusive?

This code tries to solve the problem using Z3 given as input N, G1, G2, where
    N = size of circle
    B1 = lower bound
    B2 = upper bound

Z3 is unable to solve it for the original constraints
    N = 100, B1 = 30, B2 = 50
at least within a 1 hour timeout. But Z3 can be used to solve it for smaller
constraints, e.g.
    N = 10, B1 = 3, B2 = 5

SPOILER: The problem turns out to be impossible. See proof.md for a solution.
"""

import z3

def print_result(solver):
    print(f"Constraints: {solver.assertions()}")
    result = str(solver.check())
    print(f"Result: {result}")
    if result == 'sat':
        # Sort model assignment list before printing
        model = solver.model()
        model_list = [f"{d.name()} = {model[d]}" for d in model.decls()]
        print(f"Model: {sorted(model_list)}")

def in_range(var, lb, ub):
    return z3.And(var >= lb, var < ub)

def solve_circle_problem(N, B1, B2):
    # Initialize
    solver = z3.Solver()
    vars = [z3.Int(f"n{i:0>2}") for i in range(N)]
    # Permutation of 1 through N
    solver.add(z3.And([in_range(vars[i], 1, N + 1) for i in range(N)]))
    solver.add(z3.And([
        vars[i] != vars[j] for i in range(N) for j in range(N) if i < j
    ]))
    # Difference between adjacent values is in [B1, B2]
    solver.add(z3.And([
        z3.Or(
            in_range(vars[i] - vars[(i + 1) % N], B1, B2 + 1),
            in_range(vars[(i + 1) % N] - vars[i], B1, B2 + 1),
        )
        for i in range(N)
    ]))
    # Print and then solve the constraints
    print_result(solver)

## Fast (less than a second)
solve_circle_problem(10, 3, 5) # unsat
# solve_circle_problem(10, 2, 5) # sat
# solve_circle_problem(12, 4, 6) # unsat
# solve_circle_problem(12, 3, 6) # sat

## Infeasible (doesn't terminate in an hour)
# solve_circle_problem(20, 6, 10) # unsat
# solve_circle_problem(100, 30, 50) # unsat
