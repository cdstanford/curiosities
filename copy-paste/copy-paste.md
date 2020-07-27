# Copy Paste Problem: Technical Development

Define the "score" of a sequence of key presses to be the resulting number of messages in the buffer at the end, and the "cost" to be the length of the sequence.

Define a "good" sequence to be one such that there is no sequence with a strictly lower cost and a strictly larger score. For example, `ACVVVVVVVVV` (cost 11, score 9) is good, because although there is a shorter sequence with the same score (`ACVVVACVVV`), and a sequence of the same length (11) with a larger score (`ACVVVACVVVV`), there is no shorter sequence with a strictly larger score.

All good sequences look like

    ACV...ACV...ACV...

for some number of Vs each time. We can abbreviate this as

    [a_1, a_2, ..., a_n]

where a_i >= 1 is the number of Vs, and we derive

    cost([a_1, ..., a_n]) = a_1 + ... + a_n + 2n
    score([a_1, ..., a_n]) = a_1 * ... * a_n

We can also rule out that `a_i >= 10`: for example, `[3, 4]` is striclty better than `[10]` because it has cost 11 and score 12 compared to cost 12 and score 10. In general, `[n, n + 1]` is strictly better than `[2n + 4]` for `n >= 3`, because `n^2 + n > 2n + 4`. And `[n, n]` is strictly better than `[2n + 3]` for `n >= 4`, because `n^2 > 2n + 3`.
