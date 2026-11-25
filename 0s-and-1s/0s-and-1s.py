"""
What is the smallest integer n > 1 that can be expressed using only 0s and 1s in bases
2, 3, ..., b ?

b = 2: 2
b = 3: 3
b = 4: 4
b = 5: ?
"""

import z3

# max # of digits
D_MAX = 25

def smallest_base(b):
    # d = num digits in base 2
    for d in range(1, D_MAX + 1):
        n = z3.Int("n")
        digits = {}
        for i in range(2, b + 1):
            digits[i] = [
                # n in base i, digit j
                z3.Int(f"n_{i}_{j}")
                for j in range(d)
            ]

        # Base constraint
        n_constr = [n > 1]

        # All digits 0 or 1
        zero_one = []
        for i in range(2, b + 1):
            for j in range(d):
                zero_one.append(z3.Or(digits[i][j] == 0, digits[i][j] == 1))

        # Digit sums
        digit_sums = []
        for i in range(2, b + 1):
            sum = z3.Sum([
                digits[i][j] * (i ** j)
                for j in range(d)
            ])
            digit_sums.append(sum == n)

        # Combine constraints
        constrs = n_constr + zero_one + digit_sums

        # Solve
        solver = z3.Solver()
        solver.add(z3.And(constrs))
        result = solver.check()
        if result == z3.sat:
            # Now, find minimum solution
            while result == z3.sat:
                model = solver.model()
                n_sol = model[n]
                solver.add(n < n_sol)
                result = solver.check()

            assert result != z3.unknown, "Error: unexpected Z3 failure in minimization step"

            print(f"b = {b}: solution n = {n_sol}")
            return
        elif result == z3.unknown:
            print(f"b = {b}: UNKNOWN (tried up to {d} digits)")
            return

    print(f"b = {b}: no solution found (tried up to {D_MAX} digits)")


B_MAX = 7

for b in range(2, B_MAX + 1):
    smallest_base(b)
