# Filler

An algorithmic game where two robots compete to fill a grid with randomly generated pieces.

## What is Filler?

Filler is a turn-based strategy game played on a 2D grid called "the Anfield." Two players receive random pieces from the game engine and must place them strategically to occupy the maximum territory. The player who covers the largest area wins.

## Game Rules

- Each player takes turns placing pieces on the grid
- New pieces must overlap exactly one cell with your existing territory
- Players cannot overlap opponent's pieces
- Game ends when no player can place a piece
- Winner is determined by total territory occupied

## The Anfield

The battlefield is a 2D grid with starting positions for each player:

```
..............................
..............................
..$...........................  ← Player 2 start
..............................
..............................
..............................
..............................
..............................
..............................
..............................
..............................
...........................@..  ← Player 1 start
..............................
..............................
```

## Game Pieces

The game engine provides random pieces of various shapes and sizes:

```
Piece 2x2:        Piece 5x4:        Piece 6x3:
.#                .##..             .##...
#.                .##..             ###...
                  ..#..             #..#..
                  ...#.
```

## Players

- **Player 1**: Represented by `@` (territory) and `a` (last piece)
- **Player 2**: Represented by `$` (territory) and `s` (last piece)

## Setup

1. Extract the provided docker_image folder
2. Build the Docker image:
   ```bash
   docker build -t filler .
   ```
3. Run the container:
   ```bash
   docker run -v "$(pwd)/solution":/filler/solution -it filler
   ```

## Running the Game

```bash
./game_engine -f maps/map01 -p1 robots/bender -p2 robots/terminator
```

## Game Engine Options

- `-f`: Map file path
- `-p1`: Player 1 robot path  
- `-p2`: Player 2 robot path
- `-q`: Quiet mode
- `-t`: Timeout in seconds (default: 10)
- `-s`: Random seed number

## Input/Output Format

**Input example:**
```
$$$ exec p1 : [robots/bender]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
...
Piece 4 1:
.OO.
```

**Output format:**
```
X Y
```

## Objective

Create a robot that can beat the provided opponents by implementing strategic piece placement algorithms.
