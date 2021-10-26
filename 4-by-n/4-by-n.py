"""
Partial solution explorer for the 4 by N grid problem for finite n.

Problem statement:
Consider a 4 by N grid of dots. Each dot can be connected to any subset of its 4 neighbors: (up down right left) by an edge. Can you connect all the dots so that each dot has either 1 or 3 edges coming out of it and there are no cycles? Either find an N for which there is an example or explain why it is impossible.

This file does not explore a full solution as it does not encode the "connected" and "no cycle" constraints -- these require recursive/transitive closure constraints, and could be done nicely with a tool like Datalog or Alloy. Instead, it just provides a way of getting example solutions to look at, ignoring those constraints.
"""

import z3

# Abbreviations
def h_edge(i, j):
    return z3.Bool("h(" + str(i) + "," + str(j) + ")")
def v_edge(i, j):
    return z3.Bool("v(" + str(i) + "," + str(j) + ")")
def xor(a, b, c, d):
    return z3.Xor(z3.Xor(a, b), z3.Xor(c, d))

# Problem encoding
def solve_grid_problem(M, N):
    solver = z3.Solver()
    h_edges = [[h_edge(i, j) for j in range(N+1)] for i in range(M+1)]
    v_edges = [[v_edge(i, j) for j in range(N+1)] for i in range(M+1)]
    for i in range(M):
        for j in range(N):
            solver.add(xor(v_edges[i][j], h_edges[i][j], v_edges[i+1][j], h_edges[i][j+1]))
    for i in range(M+1):
        solver.add(z3.Not(h_edges[i][0]))
        solver.add(z3.Not(h_edges[i][N]))
        solver.add(z3.Not(v_edges[i][N]))
    for j in range(N+1):
        solver.add(z3.Not(v_edges[0][j]))
        solver.add(z3.Not(v_edges[M][j]))
        solver.add(z3.Not(h_edges[M][j]))
    print(solver.assertions())
    result = str(solver.check())
    print(result)
    if result == "sat":
        print(solver.model())

if __name__ == "__main__":
    solve_grid_problem(4, 4)
