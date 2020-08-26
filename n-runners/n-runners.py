"""
A circular racetrack has N runners on it, all running at distinct
constant speeds in the same direction. There is only one spot along
the track (say, the starting line) where any runner is allowed to
pass any other runner; if two runners "collide" at any other point
along the circle, then the race is over and everyone stops.
For which N is it possible for N runners to run this way indefinitely?

The original problem asked for N = 10.
This script encodes the problem in the Z3 Theorem Prover.
It only works for up to N = 4, where it successfully finds a set of
possible runners; for a larger number, it thinks for a while, and
eventually says "unknown".

Example solution obtained by Z3 when N = 4:
    speed0 = 6,
    speed1 = 8,
    speed2 = 9,
    speed3 = 12

Notes on problem encoding:
    We observe that for any two runners at speeds r, s (in laps / hour),
    they meet every 1 / |s - r| hours. This means that r / |s - r| (the
    distance traveled in laps) must be a positive integer. We can also
    drop the absolute value and simply state that there is some integer n
    (possibly negative) such that r = (s - r) * n.
    This condition, for every pair of speeds, is sufficient to imply the
    constraints in the original problem, as long as we additionally state
    that all speeds are positive integers (in particular, not zero).
    (That they are nonzero rules out n = 0 and also means they must be
    distinct, from r = (s - r) * n.)

Proof that there is a solution for all N:
    We proceed by induction.
    Suppose that there is a solution with runner speeds r_1, r_2, ..., r_N,
    and assume WLOG that r_i are all positive integers. Let
        R = LCM(r_1, r_2, ..., r_N)
    and consider the set of N+1 positive integers
        R, R + r_1, R + r_2, ..., R + r_n.
    We claim that this set of runner speeds works. First, consider the
    pair of speeds (R + r_i) and (R + r_j): their difference is (r_i - r_j).
    This divides r_i and r_j by inductive hypothesis, and it divides R
    because it divides r_i (since R is the LCM), so it divides (R + r_i)
    and (R + r_j). Second, consider the pair of speeds R and (R + r_i).
    The difference is r_i, which divides R since it is the LCM,
    so it divides R and R + r_i. This completes the inductive step.
    Finally, for the base case we take a single runner with speed 1, and
    this completes the proof.
"""

import z3

"""
Solve the runner problem for N runners.
"""
def solve_runner_problem(N):
    solver = z3.Solver()

    # Assign a positive integer speed to each runner.
    # This is WLOG since the ratio of any two runners' speeds is rational.
    speeds = [z3.Int("speed" + str(i)) for i in range(N)]
    for s in speeds:
        solver.add(s > 0)

    # For any pair of speeds r and s, r / (s - r) is an integer.
    for i in range(N):
        for j in range(i+1, N):
            n_i_j = z3.FreshInt()
            solver.add(speeds[i] == (speeds[j] - speeds[i]) * n_i_j)

    # Print and then solve the constraints.
    print(f"Constraints: {solver.assertions()}")
    result = str(solver.check())
    print(f"Result: {result}")
    if result == 'sat':
        print(f"Model: {solver.model()}")

"""
When run from the command line: try with 1, 2, 3, 4, and 5 runners.
"""
for N in range(1, 6):
    print(f"========== Number of runners: {N} ==========")
    solve_runner_problem(N)
