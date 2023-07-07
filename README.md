# Rust SAT Polyomino Solver
This is a SAT solver for polyomino puzzles written in Rust. It is based on the [varisat](https://github.com/jix/varisat) SAT solver.

## Why?
Why not. I am at the time of writing of this file learning Rust and I thought using SAT to solve polyomino puzzles would be a fun project.

## How?
You can read more on how the problem can be translated into SAT [here](https://www.fishlet.com/2022/01/21/pentomino/).

## Examples
The first example is with tromino. You can see that that thet SAT solution is found in 134.83µs. 
```
# cargo run --example tromino
time elapsed: 286.42µs
81 vars
time elapsed: 246.29µs
217 clauses
time elapsed: 134.83µs
Result: Ok(true)
time elapsed: 144.75µs
ABB
AB 
A  
Pieces positioned: [[(0, 0), (1, 0), (2, 0)], [(0, 1), (0, 2), (1, 1)]]
```

The second example is with tetronimo. You can see that that thet SAT solution is found in 97.76s. 
```
# cargo run --example tetromino
time elapsed: 50.34ms
24109 vars
time elapsed: 6.58s
6925919 clauses
time elapsed: 91.76s
Result: Ok(true)
time elapsed: 4.40s
DFFFF
DDEEE
HDJJE
HHJBB
HIJBB
CIIGG
CCIGG
CAAAA
Pieces positioned: [[(7, 1), (7, 2), (7, 3), (7, 4)], [(3, 3), (3, 4), (4, 3), (4, 4)], [(5, 0), (6, 0), (6, 1), (7, 0)], [(0, 0), (1, 0), (1, 1), (2, 1)], [(1, 2), (1, 3), (1, 4), (2, 4)], [(0, 1), (0, 2), (0, 3), (0, 4)], [(5, 3), (5, 4), (6, 3), (6, 4)], [(2, 0), (3, 0), (3, 1), (4, 0)], [(4, 1), (5, 1), (5, 2), (6, 2)], [(2, 2), (2, 3), (3, 2), (4, 2)]]
```

The pentomino puzzle is a classic polyomino puzzle. The goal is to place all 12 pentominoes into a 6x10 grid. First we run it with flips and it runs in 52.09s.
```
# cargo run --example pentomino_flips
time elapsed: 307.12ms
125417 vars
time elapsed: 130.80s
126979357 clauses
time elapsed: 52.09s
Result: Ok(true)
time elapsed: 121.43s
KKKIII
FKBIIL
FKBBBL
FHHHBL
FFGHHL
JGGGEL
JJGEEE
AJJEDD
ACCCCD
AAACDD
Pieces positioned: [[(7, 0), (8, 0), (9, 0), (9, 1), (9, 2)], [(1, 2), (2, 2), (2, 3), (2, 4), (3, 4)], [(8, 1), (8, 2), (8, 3), (8, 4), (9, 3)], [(7, 4), (7, 5), (8, 5), (9, 4), (9, 5)], [(5, 4), (6, 3), (6, 4), (6, 5), (7, 3)], [(1, 0), (2, 0), (3, 0), (4, 0), (4, 1)], [(4, 2), (5, 1), (5, 2), (5, 3), (6, 2)], [(3, 1), (3, 2), (3, 3), (4, 3), (4, 4)], [(0, 3), (0, 4), (0, 5), (1, 3), (1, 4)], [(5, 0), (6, 0), (6, 1), (7, 1), (7, 2)], [(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)], [(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)]]
```

Then we run it finally without flips and it runs in just under 5 hours all singlethreaded.
```
# cargo run --example pentomino_flips
time elapsed: 139.11ms
81741 vars
time elapsed: 52.74s
53916967 clauses
time elapsed: 17713.95s
Result: Ok(true)
time elapsed: 51.17s
IIIGDD
IIGGGD
FFEGDD
FEEEAL
FBBEAL
FBAAAL
BBHHHL
KHHJJL
KKKCJJ
KCCCCJ
```

# Conclusion
This was a fun project. Later I will try constrained integer programing to solve polyomino puzzles. Would I recommend using SAT to solve polyomino puzzles? No. It is too slow. 

