# The Game of Life
## Brandon Zupan
### Rust

An implementation of the classic game, *Conway's Game of Life*. 

### Rules
- A new cell is born on an empty square if is surrounded by exactly three neighboring cells
- A new cell dies of overcrowding if it has four or more neighbors
- A cell dies of lineliness if it has zero or one neighbor
- Any live cell with two or three live neighbors lives, unchanged, to the next generation


The initial state of the game will be held in a text file. The format will be sequences of ones and zeros (characters). A '1' means that a cell is alive in that square. 

### Example world
```
00000000100000001010
00000000010000001001
11100000010100000010
10100100101010101010
00101010010010101000
```

The status of the grid is printed with an asterisk (\*) denoting alive and an period (.) denoting dead.  

The name of the input file and number of iterations are passed as command line parameters. 
```
<linux_prompt>$ ./life starting_grid.txt 250
```
Where the name of the executable program is "life", the input file is "starting_grid.txt", and the number of iterations is 250.


This project was adapted from a similar programming assignment that I had when learning C.  I have not disclosed the name of the class or professor to prevent people in the class finding this and plagiarizing.  
