"""
Problem: find and count the number of rigs with 3 elements.

A "rig" (also known as a semiring) is a ring without negation, i.e. a
structure with +, *, 0, and 1 such that:
- + and * are commutative and associative
- 0 is an identity for + and absorbing for *
- 1 is an identity for *
See: https://en.wikipedia.org/wiki/Semiring

I originally wrote this code around October 2018.
"""

def associative(table):
    n = len(table)
    for x in range(n):
        for y in range(n):
            for z in range(n):
                val1 = table[x][table[y][z]]
                val2 = table[table[x][y]][z]
                if val1 != val2:
                    return False
    return True

def distributive(add_table, mul_table):
    n = len(add_table) # = len(mul_table)
    for x in range(n):
        for y in range(n):
            for z in range(n):
                val1 = add_table[mul_table[x][y]][mul_table[x][z]]
                val2 = mul_table[x][add_table[y][z]]
                if val1 != val2:
                    return False
    return True

def print_table(table):
    n = len(table)
    for i in range(n):
        s = "  "
        for j in range(n):
            s += " "
            s += str(table[i][j])
        print(s)

def print_example(add_table, mul_table):
    print("Found example:")
    print(" +")
    print_table(add_table)
    print(" *")
    print_table(mul_table)

def print_skipped(reason):
    print(f"Skipped example: {reason}")

def rigs_size_3():
    # Return all rigs of size 3.
    # 0, 1, 2. Need to define 1+1=a, 1+2=2+1=b, 2+2=c, and 2*2=r.
    count = 0
    for a in range(3):
        for b in range(3):
            for c in range(3):
                for r in range(3):
                    add_table = [[0,1,2], [1,a,b], [2,b,c]]
                    mul_table = [[0,0,0], [0,1,2], [0,2,r]]
                    # Immediately commutative. Check associativity of +, *
                    # (* should be always though), and distributivity.
                    if associative(add_table):
                        if associative(mul_table):
                            # Add and mult are associative
                            if distributive(add_table, mul_table):
                                # GENUINE EXAMPLE
                                count += 1
                                print_example(add_table, mul_table)
                            else:
                                print_skipped("* not distributive over +")
                        else:
                            print_skipped("* not associative")
                    else:
                        print_skipped("+ not associative")
    print("Total number of examples:", count)
rigs_size_3()
