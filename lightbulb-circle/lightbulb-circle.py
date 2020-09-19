"""
100 lightbulbs are arranged in a circle, and all lightbulbs are initially
off. For some fixed integer k (1<=k<=49), you can iteratively select two
lightbulbs which are (a) both off, and (b) distance exactly k lightblubs
apart, and turn both of them on. For which values of k is it possible to
turn all 100 lightbulbs on?

We write a solution in general where 100 is replaced by N, and
1 <= K <= (N / 2).
"""

import itertools
import z3

result_to_str = {
    "sat": "possible", "unsat": "impossible", "unknown": "unknown"
}

"""
Solve the lightbulb problem for N lightbulbs and allowed distance K.
"""
def solve_lightbulb_problem(N, K):
    solver = z3.Solver()

    # Assign a Boolean variable to each lightbulb:
    # 1 if this lightbulb is selected as the *second* in a pair, 0 otherwise
    selected_bulbs = [z3.Bool("b" + str(i)) for i in range(N)]

    # Assert that every lightbulb turned on eactly once
    for i in range(N):
        solver.add(z3.Xor(selected_bulbs[i], selected_bulbs[(i + K) % N]))

    return result_to_str[str(solver.check())]

"""
Solve the lightbulb problem for N lightbulbs and all distances K between
1 and N/2 (inclusive)
"""
def solve_lightbulb_problem_for_all_K(N):
    results = { "possible": [], "impossible": [], "unknown": [] }
    for K in range(1, (N // 2) + 1):
        result = solve_lightbulb_problem(N, K)
        results[result].append(K)
    return results

"""
When run from the command line: try all K for even N up to 100
"""
for N in range(0, 101, 2):
    print(f"========== Number of lightbulbs: {N} ==========")
    results = solve_lightbulb_problem_for_all_K(N)
    for key, value in results.items():
        print(f"{key}: {value}")
