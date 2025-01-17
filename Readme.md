# Rust Maze Solver

A command-line maze generator and solver that uses the IDA* (Iterative Deepening A-star) algorithm to find paths through randomly generated mazes.

## Demo

![MazeDemo](/resources/RustMaze.gif)

Note that the demo fails on occasion. It was from an earlier build. The actual code always works.

## Features

- Generates random mazes using a depth-first search algorithm
- Solves mazes using IDA* pathfinding
- Visualizes the solving process in real-time with color coding:
  - Blue █: Walls
  - Green •: Solution path
  - Red @: Current position being evaluated
  - Yellow ·: Previously visited positions
- Guarantees maze solvability
- Supports configurable maze sizes

## Requirements

- Rust (latest stable version)
- Terminal with color support

## Dependencies

```toml
[dependencies]
rand = "0.8"
termion = "2.0"
```

## Building

```bash
cargo build --release
```

## Usage

Run the program with a maze size argument:
```bash
cargo run -- SIZE
```
or if built:
```bash
./maze_solver SIZE
```

The SIZE argument must be:
- An odd number
- Greater than 5

Example:
```bash
cargo run -- 21  # Creates and solves a 21x21 maze
```

## Visualization

The program shows the maze-solving process in real-time:
- The solver starts from the top-left corner
- The goal is near the bottom-right corner
- Yellow dots show explored paths
- The red @ shows the current position being evaluated
- The final solution is shown in green

## Algorithm Details

### Maze Generation
- Uses a randomized depth-first search algorithm
- Ensures connectivity between start and goal
- Creates a maze with proper walls and paths
- Guarantees at least one solution exists

### Pathfinding
- Implements IDA* (Iterative Deepening A*)
- Uses Manhattan distance as the heuristic
- Explores paths in order of estimated cost
- Shows exploration progress in real-time

## Error Handling

The program will exit with an error if:
- The size argument is not a number
- The size is not odd
- The size is less than or equal to 5

## Notes

- The terminal must support ANSI escape codes for proper visualization
- The solving process might take longer for larger mazes
- Press Ctrl+C to exit at any time
