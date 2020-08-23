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
    speed1 = 8
    speed2 = 9,
    speed3 = 12

Notes on problem encoding:
    We observe that for any two runners at speeds r, s (in laps / hour),
    they meet every 1 / |s - r| hours. This means that r / |s - r| (the
    distance travelled in laps) must be a positive integer. We can also
    drop the absolute value and simply state that there is some integer n
    (possibly negative) such that r = (s - r) * n.
    This condition, for every pair of speeds, is sufficient to imply the
    constraints in the original problem, as long as we additionally state
    that all speeds are positive integers (in particular, not zero).
    (That they are nonzero rules out n = 0 and also means they must be
    distinct, from r = (s - r) * n.)
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
