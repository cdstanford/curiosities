"""
You are given a set of n objects (for n > 2), with the following properties:
1. No two objects have the same weight.
2. For any choice of two objects in the set, there exists a subset of the remaining objects such that the subset and the chosen pair have the same total weight.
What is the minimum possible value of n?
"""

import z3
import itertools

def powerset(iterable):
    # from https://docs.python.org/3/library/itertools.html#recipes
    "powerset([1,2,3]) --> () (1,) (2,) (3,) (1,2) (1,3) (2,3) (1,2,3)"
    s = list(iterable)
    return itertools.chain.from_iterable(
        itertools.combinations(s, r) for r in range(len(s)+1)
    )

"""
Solve the problem for a set of N weights.

If use_reals is set, use real weights; else use integers
"""
def solve_weights_problem(N, use_reals):
    solver = z3.Solver()

    if use_reals:
        weights = [z3.Real("w" + str(i)) for i in range(N)]
    else:
        weights = [z3.Int("w" + str(i)) for i in range(N)]

    for i in range(N):
        # Weights positive
        solver.add(weights[i] > 0)
        for j in range(i+1, N):
            # Weights not equal
            solver.add(weights[i] != weights[j])
            # Sum of these is equal to the sum of some other subset
            others = weights[:i] + weights[i+1:j] + weights[j+1:]
            sums = [z3.Sum(otherset) for otherset in powerset(others)]
            solver.add(z3.Or([weights[i] + weights[j] == sum for sum in sums]))

    # Solve the constraints.
    ## Uncomment to print assertions
    # print(f"Constraints: {solver.assertions()}")
    result = str(solver.check())
    print(f"    Result: {result}")
    if result == 'sat':
        print(f"    Model: {solver.model()}")
    return result

"""
Solve the overall problem: try with successively higher numbers of weights.
"""

TRY_UPTO = 6

results_int = { "sat": [], "unsat": [], "unknown": [] }
results_real = { "sat": [], "unsat": [], "unknown": [] }
for N in range(1, TRY_UPTO + 1):
    print(f"========== Number of weights: {N} ==========")

    print("Integer weights: ")
    result_int = solve_weights_problem(N, False)
    print("Real weights: ")
    result_real = solve_weights_problem(N, True)

    results_int[result_int].append(N)
    results_real[result_real].append(N)

## Uncomment for summary
# print("========== Summary ==========")
# print("Integer weights: ")
# for key, value in results_int.items():
#     print(f"    {key}: {value}")
# print("Real weights: ")
# for key, value in results_real.items():
#     print(f"    {key}: {value}")
