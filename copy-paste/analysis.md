# Copy Paste Problem: Technical Development

## Optimal sequences

Define the "score" of a sequence of key presses to be the resulting number of messages in the buffer at the end, and the "cost" to be the length of the sequence.

Define a sequence to be:
- *Cost-optimal* if there is no sequence with a strictly lower cost, with at least the same score. For example, `ACVVVVVVVV` is cost-optimal even though it is not score-optimal, because it defeats the best score achievable with sequences of cost 9 or less, which is 7.
- *Score-optimal* if there is no sequence with a strictly higher score, for at most the same cost. For example, `ACVVVACVVV` is score-optimal because 9 is the best possible score for a cost of at most 10, but `ACVVVVVVVV` is not score-optimal, because 8 is not as good as the previous sequence's score of 9 (for the same cost). Also, `ACV` is score-optimal because although there are shorter sequences of score 1, there is no sequence of strictly higher score.

Then, say that a sequence is
- *Sub-optimal* if it is neither score- nor cost-optimal;
- *Semi-optimal* if it is not sub-optimal -- i.e., it is either score- or cost-optimal; and
- *Optimal* if it is both score- and cost-optimal. (Optimal sequences are also semi-optimal).

For the basic search problem where we don't have to return all results,
searching over optimal sequences is enough. There will always be an optimal sequence that beats whatever candidate other solutions in either score or cost.

However, to handle the harder problem where we have to return *all* results that are minimal cost for a certain target score, we need to search over all cost-optimal sequences, even if they are not score-optimal. And conversely, to handle the flipped problem where we return all results that are maximimum score given a certain allowed cost, we would need to search over all score-optimal sequences, even if they are not cost-optimal.

There are only a finite number of score-optimal sequences that aren't cost-optimal. Since such a sequence is not cost-optimal, there must be a shorter sequence with the same score (but not better, or it would not be optimal). Now what can this shorter sequence look like? It can't end in a `V`, or else it could be improved in score by adding a `V` thus disproving the original sequence's score-optimality. Similarly it can't end in a `V` followed by a bunch of `A` and `C`s, or these `A` and `C`s could be replaced with a `V`. So it has no `V`s, just a sequence of `A` and `C`s, and it has score `1`. So the original sequence has score `1` as well; in order to not be defeated by `ACVV` then it must be cost at most 3. The possibilities here are all sequences of 1 to 3 characters (0 would be cost-optimal) with score 1; there are 36 such sequences total.

**TL;DR:** All the problems we are interested in reduce to searching only over cost-optimal sequences, so we can ignore all others.

## Representing cost-optimal sequences

All cost-optimal sequences look like

    ACV...ACV...ACV...

for at least two Vs each time. (This is because we can remove `A`s that are not immediately followed by `C`, then remove `C`s that are not preceded by `A`, and these steps only increase score. Then finally, remove `AC` or `ACV` if it is either at the end or followed by `AC`.) We can abbreviate this as

    [a_1, a_2, ..., a_n]

where a_i >= 2 is the number of Vs, and we derive

    cost([a_1, ..., a_n]) = a_1 + ... + a_n + 2n
    score([a_1, ..., a_n]) = a_1 * ... * a_n

We can also rule out that `a_i >= 9`: for example, `[3, 3]` is better than `[9]` because it has cost 10 and score 9 compared to cost 11 and score 9. In general, `[n, n + 1]` is strictly better than `[2n + 4]` for `n >= 3`, because `n^2 + n >= 2n + 4`. And `[n, n]` is strictly better than `[2n + 3]` for `n >= 3`, because `n^2 >= 2n + 3`.

Since all `a_i` are between 2 and 8 and the order does not matter, we represent the whole cost-optimal sequence as a 7-tuple

    (v_2, v_3, ..., v_8)

where `v_j` is the number of `a_i` such that `a_i = j`. On this final representation, we derive

    cost((v_2, v_3, ..., v_8)) = 4 * v_2 + 5 * v_3 + ... + 11 * v_8
    score((v_2, v_3, ..., v_8)) = 2^(v_2) * 3^(v_3) * ... * 9^(v_8)

## Characterizing cost-optimal sequences

Asymptotically, 4s (v_4) are the best, because they give the most "bang" for the "buck", where bang is ln(score) and buck is cost. Score is multiplicative, so log(score) is additive. Then 4s have bang / buck = ln(4) / 6 = 0.23104..., which turns out to be better than the others (e.g. ln(3) / 5 = 0.219... and ln(5) / 7 = .229...). The actual optimum is the maximum of ln(x) / (x + 2) which can be derived to be .23152... at x = e^(W(2/e) + 1) where W is the lambert W function.

Therefore we suspect that all cost-optimal sequences should only have finitely many values that aren't 4s. Specifically, we can show that any cost-optimal `(v_2, v_3, v_4, v_5, v_6, v_7, v_8)` satisfies:

- `v_2 <= 1` because 4 is better than 2, 2
- `v_3 <= 3` because 4, 4, 5 is better than 3, 3, 3, 3
- `v_5 <= 30` because 36 4s are better than 31 (!) 5s.
- `v_6 <= 4` because 4, 4, 4, 5, 5, 5 is better than 6, 6, 6, 6, 6
- `v_7 <= 2` because 4, 4, 5, 5 is better than 7, 7, 7
- `v_8 <= 1` because 4, 4, 4 is better than 8, 8

where "better" means strictly lower cost and at least as high score.

Multiplied together there are 2 * 4 * 31 * 5 * 3 * 2 = 7440 possibilities here for all the v_i other than v_4, which is feasible to write out explicitly.

## Algorithm to find all minimum-cost sequences that score a given target

We search over all 7440 possibilities for the values of the tuple other than 4, and then fill in as many 4s as needed to get above the target. Among all these 7440 candidates, we know the minimum cost occurs, and we throw out all that have smaller cost, and print out the rest.
