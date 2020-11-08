"""
You are given 5 positive real numbers, a<b<c<d<e, with the property that the total sum of their squares is equal to the total sum of their pairwise products. 
- Prove that we can choose an ordered triplet of numbers from this set (x,y,z) such that x<y<z, and such that it is not possible to form a triangle with these side lengths.
- Prove that the number of such ordered triplets is at least 6.

Z3 FAILED
This is another one that Z3 was unable to solve -- due to the quadratic
(not decidable) real number arithmetic constraints.
"""

import z3

s = z3.Optimize()
a, b, c, d, e = z3.Reals("a b c d e")

# Hard constraints
s.add(0 < a, a < b, b < c, c < d, d < e)
sum_squares = a*a + b*b + c*c + d*d + e*e
sum_pairs = a*b + a*c + a*d + a*e + b*c + b*d + b*e + c*d + c*e + d*e
s.add(sum_squares == sum_pairs)

# Soft constraints
# Each one states that the triangle inequality holds for a particular triple
s.add_soft(a + b < c)
s.add_soft(a + b < d)
s.add_soft(a + b < e)
s.add_soft(a + c < d)
s.add_soft(a + c < e)
s.add_soft(a + d < e)
s.add_soft(b + c < d)
s.add_soft(b + c < e)
s.add_soft(b + d < e)
s.add_soft(c + d < e)

# Check: i.e. find the maximum number of constraints that are satisfiable
# print(s.assertions())
# print(s.objectives())
print(s.check())
if str(s.check()) == 'sat':
    print(s.model())
    num_not_sat = s.model().eval(s.objectives()[0])
    print(f"Num triangles guaranteed: {num_not_sat}")
    assert int(str(num_not_sat)) >= 6
if str(s.check()) == 'unknown':
    print(s.reason_unknown())
