# Solution for general N and K

We claim that the problem is solvable if and only if
N / GCD(N, K)
is even.

First we show this condition is necessary. If not, consider the set S of lightbulbs that are multiples of GCD(N, K). The number of lightbulbs in S is N / GCD(N, K). Every move either turns on two lightbulbs in S, or doesn't affect S. So the number of these lightbulbs in S that are on at any point is even, and if N / GCD(N, K) is odd, it follows that we can never turn every lightbulb in S.

Then we show it is sufficient. If N / GCD(N, K) is even, then let d = GCD(N, K). Note that 2*d divides N. For the first lightbulb in a selected pair, choose every lightbulb i where i is equivalent to 0, 1, 2, ..., or d-1 mod 2d. We claim that every lightbulb is turned on exactly once this way. Note that 2d divides N but 2d does not divide K, so K is equivalent to d mod 2d. That means that whenever we select a lightbulb i in a pair with i equivalent to 0, 1, 2, ..., or d-1, the second lightbulb in the pair i + K is equivalent to d, d+1, d+2, ..., or 2d-1. Conversely if i is equivalent to d, d+1, d+2, ..., or 2d-1, then i - K was selected as the first in a pair so i must have been selected as the second lightbulb in a pair. So each lightbulb is selected as either the first or second in a pair, and so is turned on exactly once.

# Solution for N = 100

For N = 100 the above implies that K is possible if and only if K is not a multiple of 4.
