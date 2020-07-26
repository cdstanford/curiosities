"""
What is the optimal way to copy and paste to create a long message of many
copies of the same character?

Problem statement:
    Given input a target number t, determine *all* minimum-length sequences
    of key presses to get a message of *at least* t of the same character
    in the message buffer, starting from a message buffer of 1 character
    (not selected) and a clipboard of 0 characters.

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
    Expected output:
        ACVVVACVVV
        ACVVVVVVVV
    Note that the first produces 9 while the second produces 8, but both have
    the minimum length of 10 key presses,
    so both should be returned as valid answers.

    ADDENDUM:
    Answers only need to be printed "up to equivalence" where we define that
    V^mACV^n is equivalent to V^nACV^m.
    (Question: does this change the minimum complexity?)
"""
