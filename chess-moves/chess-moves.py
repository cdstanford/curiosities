"""
What is the minimum number of moves required in chess to move all pieces up one square?

see
    chess-moves.png
    From: https://www.reddit.com/media?url=https%3A%2F%2Fi.redd.it%2Faemmq2pzk5dg1.png

This program does not terminate within a few minutes.
It gets up to around n == 11 before diverging.
"""

import z3

def valid_square(pos):
    i, j = pos
    return z3.And(i >= 1, i <= 8, j >= 1, j <= 8)

def valid_pawn(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    # simplify for the purposes of this problem
    # allows passing move
    return z3.And(i1 == i2, z3.Or(j1 == j2, j2 == j1 + 1))

def valid_rook(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    # allows passing move
    return z3.Or(i1 == i2, j2 == j2)

def abs(x):
    return z3.If(x > 0, x, -x)

def valid_bishop(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    # allows passing move
    return abs(i1 - i2) == abs(j1 - j2)

def valid_knight(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    case1 = z3.And(abs(i1 - i2) == 1, abs(j1 - j2) == 2)
    case2 = z3.And(abs(i1 - i2) == 2, abs(j1 - j2) == 1)
    case3 = z3.And(i1 == i2, j1 == j2)
    return z3.Or(case1, case2, case3)

def valid_queen(pos1, pos2):
    return z3.Or(
        valid_rook(pos1, pos2),
        valid_bishop(pos1, pos2),
    )

def valid_king(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    return z3.And(abs(i1 - i2) <= 1, abs(j1 - j2) <= 1)

def stationary(pos1, pos2):
    i1, j1 = pos1
    i2, j2 = pos2
    return z3.And(i1 == i2, j1 == j2)

def is_valid_move(l1, l2):
    # len(l1) == 16
    # len(l2) == 16
    constraints = []

    # Each piece's move is valid
    for i in range(8):
        constraints.append(valid_pawn(l1[i], l2[i]))
    constraints.append(valid_rook(l1[8], l2[8]))
    constraints.append(valid_rook(l1[9], l2[9]))
    constraints.append(valid_knight(l1[10], l2[10]))
    constraints.append(valid_knight(l1[11], l2[11]))
    constraints.append(valid_bishop(l1[12], l2[12]))
    constraints.append(valid_bishop(l1[13], l2[13]))
    constraints.append(valid_king(l1[14], l2[14]))
    constraints.append(valid_queen(l1[15], l2[15]))

    # Only one piece moves
    possibilities = []
    for i in range(16):
        possibility = []
        for j in range(16):
            if j != i:
                possibility.append(stationary(l1[j], l2[j]))
        possibility.append(valid_square(l1[i]))
        possibility.append(valid_square(l2[i]))
        possibilities.append(z3.And(possibility))
    constraints.append(z3.Or(possibilities))

    return z3.And(constraints)

def start_board():
    return [
        # pawns
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (5, 2),
        (6, 2),
        (7, 2),
        (8, 2),
        # rooks
        (1, 1),
        (8, 1),
        # knights
        (2, 1),
        (7, 1),
        # bishops
        (3, 1),
        (6, 1),
        # king
        (4, 1),
        # queen
        (5, 1),
    ]

def is_start_board(l):
    board = start_board()
    return z3.And([
        l[i][j] == board[i][j]
        for i in range(16)
        for j in range(2)
    ])

def is_end_board(l):
    board = end_board()
    return z3.And([
        l[i][j] == board[i][j]
        for i in range(16)
        for j in range(2)
    ])

def end_board():
    return [
        # pawns
        (1, 3),
        (2, 3),
        (3, 3),
        (4, 3),
        (5, 3),
        (6, 3),
        (7, 3),
        (8, 3),
        # rooks
        (1, 2),
        (8, 2),
        # knights
        (2, 2),
        (7, 2),
        # bishops
        (3, 2),
        (6, 2),
        # king
        (4, 2),
        # queen
        (5, 2),
    ]

def valid_board_sequence(l):
    constraints = []
    n = len(l)
    constraints.append(is_start_board(l[0]))
    constraints.append(is_end_board(l[n-1]))
    for i in range(n - 1):
        constraints.append(is_valid_move(l[i], l[i+1]))

    return z3.And(constraints)

def board(idx):
    # generate a board with Z3 variables using index idx
    l = []
    for i in range(16):
        x = z3.Int(f"x{i}_{idx}")
        y = z3.Int(f"y{i}_{idx}")
        l.append((x, y))
    return l

def solution(n):
    boards = [board(0)]
    for i in range(1, n + 1):
        boards.append(board(i))
    return valid_board_sequence(boards)

if __name__ == "__main__":
    print("===== Solving minimum # of chess moves puzzle =====")
    for n in range(0, 30):
        soln = solution(n)
        print(f"=== for n = {n} ===")
        z3.solve(soln)
