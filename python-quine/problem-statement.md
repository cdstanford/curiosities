# Python Quine

Write a Python program that prints itself as output when
run on the command line, i.e. a [Quine](https://en.wikipedia.org/wiki/Quine_(computing)).

(I wrote this program in October 2015, while thinking about the proof of the recursion theorem for Turing machines from Sipser. The theorem states that Turing machines can be assumed WLOG to have a copy of their own source code. Or formally, if you write a Turing machine M that computes a function `(x, y) ↦ f(x, y)`, you can construct a Turing machine M' that computes `y ↦ f(<M'>, y)` where `<M'>` is a description of M'.)
