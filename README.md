# Rot Lang
Rot is a language with a simple goal: to be as inherently slow as possible.\
... or at least, as slow as I can make it\
To my knowledge, its execution is inherently dependent on an NP-hard problem with factorial time complexity, but it's possible that there are other optimizations that I don't know about. 

## Approach
Looking for ways to make the objectively worst programming language, my mind immediately went to graph problems, as many of these have high time complexity. After first considering data dependency graphs and problems like graph coloring, but failing to find a good way to turn them into a programming language, I settled on something a little simpler: the shortest hamiltonian path problem. This is similar to the better known traveling salesman problem, except it doesn't require the path to start and end in the same place. I chose this approach because, like the traveling salesman problem, solutions to this problem generally take factorial time.

## Concept
Rot is expressed as a list of nodes, each given:
* an index based on their position in the program
* x, y, and z coordinates
* an expression or operation

The program is executed from the first node, following the physically shortest path through all nodes which visits each node exactly once. I'm not yet sure how path order will be implemented.

Code is self-modifying. Self-modification is used for:
* Control flow
* Data storage
* Inflated time consumption ;P

Moving nodes changes the path of the program, which can be used for loops and branches.\
The Z component of a node is used to store data, and can be easily accessed for arithmetic, I/O, etc...\
changing the Z component also changes the distances between nodes, which must be considered carefully when writing in Rot.

# Writing in Rot
A node is written as follows:
- `(x, y, z, expr)`

x, y, and z represent the initial coordinates\
expr is any expression as defined in [Expressions](#Expressions)\
A node's index is the number of nodes that came before it. The first node is node 0, the next is node 1, etc...

# Expressions
This list is incomplete, but expressions might include / look like:
- `nop`
- `exit`
- `display src`
- `displaychar src`
- `input dest`
- `inputchar dest`
- `move dest, srcx, srcy, srcz`
- `add dest, src1, src2`
- `sub dest, src1, src2`
- `multiply dest, src1, src2`
- `divide dest, src1, src2`

destinations must be nodes. How the operation affects a node depends on which operation is being done.
sources can be nodes, number literals, or just an `x` for optional inputs/outputs that are to be disregarded. `x` is used, for example, if you only want to modify some of the coordinates of a node using `move`.\
Node literals will be expressed as an octothorpe (`#`) followed by an integer literal.

# Examples
I don't trust myself to write actual code without an interperter, but here's a simple example that adds 9 + 10 and prints it (hopefully):
```
(0, 0, 9, nop)              // loads 9 into #0
(100, 0, 10, nop)           // loads 10 into #1
(200, 0, 0, add #2, #0, #1) // stores 9+10 in #2
(300, 0, 0, display #2)     // prints 19 to screen
```

And here's hello world:
```
(0, 0, 72, displaychar #0)
(1000, 0, 101, displaychar #1)
(2000, 0, 108, displaychar #2)
(3000, 0, 108, displaychar #3)
(4000, 0, 111, displaychar #4)
(5000, 0, 44, displaychar #5)
(6000, 0, 32, displaychar #6)
(7000, 0, 87, displaychar #7)
(8000, 0, 111, displaychar #8)
(9000, 0, 114, displaychar #9)
(10000, 0, 108, displaychar #10)
(20000, 0, 100, displaychar #11)
(30000, 0, 33, displaychar #12)
(40000, 0, 10, displaychar #13)
```
