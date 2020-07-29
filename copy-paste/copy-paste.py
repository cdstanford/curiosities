"""
What is the optimal way to copy and paste to create a long message of many
copies of the same character?

Problem statement:
    Given input a target number t, find and return one of the
    minimum-length sequences of key presses to get a message of *at least*
    t of the same character in the message buffer, starting from a message
    buffer of 1 character (not selected) and a clipboard of 0 characters.

    The solution time complexity should be as low as possible.

    The *state* is defined to be a message buffer of one or more characters,
    together with a clipboard of zero or more characters,
    where the message buffer can be either selected or deselected.
    The initial state has a message of 1 character, with 0 characters
    in the clipboard, and the message is not selected.
    Allowed key presses are:
    - Select (ctrl-A): select all characters in the message buffer
    - Copy (ctrl-C): if the message is selected, set the clipboard to be equal
      to the message buffer; otherwise, do nothing.
    - Paste (ctrl-V): if the message is selected, replace it with the clipboard;
      otherwise, append the clipboard to it.

    For simplicity, we don't allow deselecting or typing a character. (The
    former does not end up being useful, while the latter does offer
    improvements, but doesn't generalize to duplicating a message larger
    than a single character).

    Example input: 8
    Expected output: one of
        ACVVVACVVV
        ACVVACVVVV
        ACVVVVACVV
        ACVVVVVVVV
    Note that the first produces 9 while the others produce 8, but all have
    the minimum length of 10 key presses, so all are valid answers.

Hard version:
    Return *all* valid answers, instead of just one.
    To simplify this, the answers only need to be printed "up to equivalence",
    where equivalence is defined by:
        (not a V) V^m AC V^n (not a V) == (not a V) V^n AC V^m (not a V)
    for all positive integers m and n.

Time complexity:
    The solution here has constant time complexity (assuming O(1) arithmetic
    operations) using the internal representation of the solution as a tuple.
    However, printing out the solution as a string of key presses
    takes time O(log(n)).

External links on the copy-paste problem:
    https://oeis.org/A178715 (roughly equivalent problem)
    https://oeis.org/A193286 (assumes that copy deselects the characters)
    https://math.stackexchange.com/questions/483596/least-amount-of-steps-to-get-over-1000/48365
    https://codegolf.stackexchange.com/questions/38410/copy-paste-master
"""

import math
import operator
import functools

"""
Sequences of copy-pastes are represented as 9-tuples
    (v_0, v_1, ..., v_8)
where this corresponds to the sequence
    (AC)^(v_0) (ACV)^(v_1) (ACVV)^(v_2) (ACVVV)^(v_3) ... (ACVVVVVVVV)^(v_8).

See copy-paste.md for the formal development of this.

In actuality, v_0 and v_1 will always be 0, but they are included to
make indexing more natural (v_i corresponds to the number of sequences of
i Vs in a row).
"""

TUP_LEN = 9

def prod(tup):
    return functools.reduce(operator.mul, tup, 1)

def cost(tup):
    # Cost = number of key presses
    return sum([(i+2) * tup[i] for i in range(TUP_LEN)])

def score(tup):
    # Score = resulting message size
    # Note: using log score is more compact and efficient, but I'm avoiding
    # floating point and just making sure to call this only for small tuples.
    return prod(i**(tup[i]) for i in range(TUP_LEN))

def key_presses(tup):
    # Return the sequence of key presses
    return ''.join("AC" + "V" * i for i in range(TUP_LEN) for j in range(tup[i]))


def tuples_bounded_by(tup):
    """
    Generate all tuples of nonnegative integers where the coordinates are
    bounded by the given tuple.
    """
    if len(tup) == 0:
        yield ()
    else:
        yield from (
            (head,) + tail
            for tail in tuples_bounded_by(tup[1:])
            for head in range(tup[0] + 1)
        )

CANDIDATE_BOUNDS = (0, 0, 1, 3, 0, 30, 4, 2, 1)
def cost_optimal_tuple_candiates():
    """
    Generate all candidates for cost-optimal tuples where v_4 = 0.
    There are 7440 of them.

    These are used to build cost-optimal tuples later on (see copy-paste.md)
    """
    return tuples_bounded_by(CANDIDATE_BOUNDS)
assert len(CANDIDATE_BOUNDS) == TUP_LEN
assert len(list(cost_optimal_tuple_candiates())) == 7440

def get_all_solutions(target):
    """
    Print all sequences of minimum cost which reach the target or higher.
    """
    candidates = []
    for tup in cost_optimal_tuple_candiates():
        s = score(tup)
        # Add 4s until score is above target
        if target <= s:
            num_fours = 0
        else:
            num_fours = math.ceil(math.log(target / s, 4))
        candidate = tup[:4] + (num_fours,) + tup[5:]
        candidates.append(candidate)
    # Keep candidates with the min cost
    min_cost = min(cost(cand) for cand in candidates)
    candidates = [cand for cand in candidates if cost(cand) == min_cost]
    # Print solution
    print("===== Results =====")
    print(f"Min number of keypresses: {min_cost}")
    print(f"Number of inequivalent solutions: {len(candidates)}")
    print(f"Solutions as tuples:")
    for cand in candidates:
        print(f"  {cand[2:]}")
    print(f"Solutions as key presses:")
    for cand in candidates:
        print(f"  {key_presses(cand)}")

print("Target?")
while True:
    try:
        t = int(input())
        assert t >= 0
    except (ValueError, AssertionError):
        print("Target should be a nonnegative integer")
    else:
        break
get_all_solutions(t)
