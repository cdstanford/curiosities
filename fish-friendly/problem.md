# Fish-friendly grid problem

An $m \times n$ grid (with $m$ rows and $n$ columns) has some of its squares colored blue.
The grid is called *fish-friendly* if a fish can swim from from the left to the right of the grid: that is, there is a sequence of blue squares, each horizontally or vertically adjacent to the previous square, starting in the first column and ending in the last column.

How many fish-friendly $m \times n$ grids are there?

## Alternate version

For an alternate version of this problem, we can disallow right-to-left steps. For this version, comment out line 88 in `src/lib.rs`.

## References

[2022 Utah Math Olympiad Problem 6](https://utahmath.org/doc/2022UtahMathOlympiad.pdf)

[OEIS sequence A359576](https://oeis.org/A359576)

[OEIS sequence A365988](https://oeis.org/A365988)
