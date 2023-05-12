"""
Let [n] denote {0, 1, 2, ..., n - 1}.

Given n, how many subsets A of [n] are there such that A + A contains [n]?

Here X + Y is the Minkowski sum: X + Y := {x + y | x in X, y in Y}.

See OEIS:
https://oeis.org/A066062
"""

import z3

# Generic model counting methods
# see https://theory.stanford.edu/~nikolaj/programmingz3.html#sec-blocking-evaluations

def block_model(s, vars):
    m = s.model()
    s.add(z3.Or([t != m.eval(t, model_completion=True) for t in vars]))

def iter_models(s, vars):
    while "sat" == str(s.check()):
       yield s.model()
       block_model(s, vars)

def print_model(m, vars):
    print([t for t in vars if m.eval(t, model_completion=True)])

# Solution to the present problem

def covers(n, vars):
    return z3.Or([z3.And(vars[i], vars[n-i]) for i in range(n+1)])

def solve(N):
    print(f"=== {N} ===")
    solver = z3.Solver()
    vars = [z3.Bool(str(i)) for i in range(N)]
    for n in range(N):
        solver.add(covers(n, vars))
    count = 0
    for m in iter_models(solver, vars):
        # Uncomment to print solutions
        # print_model(m, vars)
        count += 1
    return count

seq = []
for N in range(16):
    ans = solve(N)
    print(F"{ans} models")
    seq.append(ans)

print("=== Answers ===")
print(seq)
