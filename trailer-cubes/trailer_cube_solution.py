"""
Trailer cube puzzle

Let's use Z3 to find out & resolve this question.

=== Trailer cube puzzle ===

How many cubes are in the trailer?

A small spoiler:
The answer may depend on which assumptions are made.
"""

"""
Imports and helper functions
"""

import z3
from helper import get_solution

"""
=== Step 1: Define variables ===

An integer x?
"""

# answer = z3.Int("x")
# solve(answer == 51) # problem?

# go based on position?
# Idea: Make a bunch of Boolean variables for whether or not a box is in a particular location

# Nested arrays are once again useful
# 3 x 3 x 7
# height, row (i), column (j)

z3_grid = [
    [
        [
            z3.Bool(f"box{h}{i}{j}") for j in range(7)
        ]
        for i in range(3)
    ]
    for h in range(3)
]

"""
=== Step 2: Define constraints ===

Top view:
- Anything at height 0 is filled?

  ^^^ implicitly making an assumption!

      (gravity is present)

Let's use the more general: one of height 0, 1, or 2 is filled
and come back to the gravity issue

Side view:
- Anything in the L shape is not filled

Back view:
- At least one box in every (depth, row) position such that the back view
  is filled.
"""

constraints = []

# Top view
for i in range(3):
    for j in range(7):
        constraints.append(z3.Or([
            z3_grid[h][i][j] for h in range(3)
        ]))

# Side view
L_SHAPE_COORDS = [(4, 2), (5, 2), (6, 1), (6, 2)]
for h in range(3):
    for j in range(7):
        if (j, h) in L_SHAPE_COORDS:
            # L-shape is NOT filled
            for i in range(3):
                constraints.append(z3.Not(z3_grid[h][i][j]))
        else:
            # remainder (outside L shape) IS filled
            # use z3.Or
            constraints.append(z3.Or([
                z3_grid[h][i][j] for i in range(3)
            ]))

# Back view
for h in range(3):
    for i in range(3):
        # using z3.Or again
        constraints.append(z3.Or([
            z3_grid[h][i][j] for j in range(7)
        ]))

"""
Gravity

For the "top" view constraint, we said we might want to assume gravity

How to encode gravity more generally?

Tracking as a separate system of constraints so that we can explore different solutions below
"""

gravity_constraints = []

for h in range(1, 3):
    for i in range(3):
        for j in range(7):
            gravity_constraints.append(z3.Implies(z3_grid[h][i][j], z3_grid[h - 1][i][j]))

"""
Other assumptions?

Other assumptions are possible.
Here are two:

Flat perspective:

We could assume the back and side views constitute a "flat" perspective
(although the top view necessarily does not):
"""

flat_constraints = []

for h in range(3):
    # Back view
    for i in range(3):
        flat_constraints.append(z3_grid[h][i][0])

    # Side view
    for j in range(7):
        if (j, h) not in L_SHAPE_COORDS:
            flat_constraints.append(z3_grid[h][0][j])

"""
Glue constraint:

Here we don't assume full gravity, but we assume the boxes can be glued to each other
and must touch the ground indirectly through some series of boxes.

This one is hard! one way to do it: ensure each box has a "distance" and is glued to a box with a smaller index,
or touches the ground.
"""

dist_grid = [
    [
        [
            z3.Int(f"dist{h}{i}{j}") for j in range(7)
        ]
        for i in range(3)
    ]
    for h in range(3)
]

glue_constraints = []

# Height 0
for i in range(3):
    for j in range(7):
        glue_constraints.append(dist_grid[0][i][j] == 0)
# Height 1, 2
for h in range(1, 3):
    for i in range(3):
        for j in range(7):
            def adjacent_cond(triple):
                _, i_, j_ = triple
                return i_ >= 0 and i_ < 3 and j_ >= 0 and j_ < 7
            adjacent = filter(
                adjacent_cond,
                [(h-1, i, j), (h, i-1, j), (h, i+1, j), (h, i, j-1), (h, i, j+1)]
            )

            glue_constraints.append(z3.Implies(
                z3_grid[h][i][j], # if there is a box at this cell ...
                z3.Or([
                    # ... then there is a box at some neighboring cell, and distance is calculated appropriately
                    z3.And(z3_grid[h_][i_][j_], dist_grid[h][i][j] == dist_grid[h_][i_][j_] + 1)
                    for h_, i_, j_ in adjacent
                ])
            ))

"""
=== Step 3: Pass the constraints to Z3 ===

Remains: ... we need total number of cubes

z3.Sum ?

How to convert Booleans to integers?

    z3.If(b, 1, 0)
"""

total_cubes = 0
for h in range(3):
    for i in range(3):
        for j in range(7):
            total_cubes += z3.If(z3_grid[h][i][j], 1, 0)

# Different specs
base_spec = z3.And(constraints)
gravity_spec = z3.And(constraints + gravity_constraints)
flat_spec = z3.And(constraints + flat_constraints)
flat_gravity_spec = z3.And(constraints + gravity_constraints + flat_constraints)
glue_spec = z3.And(constraints + glue_constraints)

"""
Pretty printing
"""

# Helper function to print solution
def pretty_print_solution(spec, constr):
    model = get_solution(z3.And(spec, constr))
    if model is None:
        print("    No solution")
        return
    print("    Height: 0         1         2")
    for i in range(3):
        box_row = "         "
        for h in range(3):
            box_row += "   "
            for j in range(7):
                box = model[z3_grid[h][i][j]]
                box_row += ('#' if box else '.')
        print(box_row)

# maximum is 51:
print("=== Maximum ===")
print("Total cubes > 51:")
pretty_print_solution(base_spec, total_cubes > 51)
print("Total cubes = 51:")
pretty_print_solution(base_spec, total_cubes == 51)

# minimum is 31 - assuming gravity applies
print("=== Gravity ===")
print("With gravity, total cubes < 31:")
pretty_print_solution(gravity_spec, total_cubes < 31)
print("With gravity, total cubes = 31:")
pretty_print_solution(gravity_spec, total_cubes == 31)

# minimum is 21 - assuming no gravity
print("=== No gravity ===")
print("Without gravity, total cubes < 21:")
print("(Note: takes about 2 minutes...)")
pretty_print_solution(base_spec, total_cubes < 21)
print("Without gravity, total cubes = 21:")
pretty_print_solution(base_spec, total_cubes == 21)

# minimum is 35 - assuming "flat" perspective
print("=== Flat perspective ===")
print("Flat perspective, total cubes < 35:")
pretty_print_solution(flat_spec, total_cubes < 35)
print("Flat perspective, total cubes = 35:")
pretty_print_solution(flat_spec, total_cubes == 35)
print("Flat perspective, with gravity, total cubes = 35:")
pretty_print_solution(flat_gravity_spec, total_cubes == 35)

# minimum is 23 - with glue
print("=== Glue ===")
print("Gravity with glue, total cubes = 23:")
print("(Note: takes up to 30 seconds...)")
pretty_print_solution(glue_spec, total_cubes == 23)
print("Gravity with glue, total cubes < 23:")
print("(Note: takes about 4 minutes...)")
pretty_print_solution(glue_spec, total_cubes < 23)

"""
python3 trailer_cube_solution.py  348.14s user 0.74s system 99% cpu 5:49.20 total

i.e. for the above particular sets of assumptions:

- the maximum number of cubes is 51
- assuming gravity is present, the minimum number of cubes is 31.
- without assuming gravity, the minimum number of cubes is 21.
- assuming a "flat" perspective, the minimum number of cubes is 35.
- with gravity but allowed to use glue, the minimum number of cubes is 23.
"""
