"""
What is the optimal way to choose a set of currency denominations?

In this problem, we define the "score" of a set of positive integer denominations (bills) as the product of:
1. The average number of bills required to make a number from 1 to 100. For example, if the denominations are 1, 5, 10, 20, and 50, the number of bills required to make 47 is 5, 1 + 1 + 5 + 20 + 20. The average turns out to be 4.22.
2. The number of bills. For our example, this is 5, so the score would be 5 * 4.22 = 21.1.

The question is, what is the minimum possible score?

Problem source: Benjamin Stanford on 2020-06-05, via email.
"""

import argparse
from typing import List

N = 100

# score function (in general, anything increasing
# in avg and numdenominations makes sense here)
def get_score(avg: float, numdenominations: int) -> float:
    return avg * numdenominations

# avg # of bills to create a random value from 1 to N
def get_avg(denoms: List[int], N: int) -> float:
    assert denoms[0] == 1, "Error: first denomination should be 1"
    bills_for = [0]
    for i in range(1, N+1):
        best = min([    
            1 + bills_for[i - d]
            for d in denoms
            if i - d >= 0
        ])
        bills_for.append(best)
    return sum(bills_for) / (len(bills_for) - 1)

# Simple command-line interface
if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'integers', type=int, nargs='+',
        help='list of denominations'
    )
    denoms = parser.parse_args().integers
    print(f"Denominations provided: {denoms}")
    avg = get_avg(denoms, N)
    score = get_score(avg, len(denoms))
    print(f"avg: {avg}; number: {len(denoms)}")
    print(f"score: {score}")
