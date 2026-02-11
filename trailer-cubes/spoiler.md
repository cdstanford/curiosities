# Spoiler: possible answers

There are at least 5 possible answers.
Or 4, if you only count different answers for the
minimum possible number of cubes.

1. Maximum solution: 51 cubes
```
Total cubes = 51:
    Height: 0         1         2
            #######   ######.   ####...
            #######   ######.   ####...
            #######   ######.   ####...
```

2. Minimum solution, with gravity: 31 cubes:
```
Total cubes = 31:
    Height: 0         1         2
            #######   #......   #......
            #######   .#.###.   .#.#...
            #######   ..#....   ..#....
```

The above were found using Z3.
The below were found manually.

3. Minimum solution, no gravity: 21 cubes
```
Total cubes = 21:
    Height: 0         1         2
            #..#.##   ..#.#..   .#.....
            .#..#.#   #..#.#.   ..#....
            ..#..##   .#..#..   #..#...
```

4. Minimum solution, allowed to use glue: 23 cubes:
```
Total cubes = 23:
    Height: 0         1         2
            ...#.##   #####..   ...#...
            ###...#   ....##.   ...#...
            ....#.#   .....#.   ####...
```

5. Minimum solution, assuming side view and back view (not top view) are a "flat" perspective:
```
Total cubes = 35
    Height: 0         1         2
            #######   #......   #......
            #######   #......   #......
            #######   ######.   ####...
```
