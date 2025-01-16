import random
import time
import os
import sys
from enum import Enum
from typing import List, Tuple, Optional, Set

class Cell(Enum):
    WALL = '#'
    PATH = ' '
    SOLUTION = '•'
    CURRENT = '@'
    VISITED = '·'

class Maze:
    def __init__(self, size: int):
        self.size = size
        self.grid = [[Cell.WALL for _ in range(size)] for _ in range(size)]
        self.start = (1, 1)
        self.goal = (size-2, size-2)
        # Set start and goal
        self.grid[1][1] = Cell.PATH
        self.grid[size-2][size-2] = Cell.PATH

    def generate(self):
        stack = [self.start]
        while stack:
            current = stack[-1]
            neighbors = self.get_unvisited_neighbors(*current)
            
            if not neighbors:
                stack.pop()
            else:
                nx, ny = random.choice(neighbors)
                self.grid[nx][ny] = Cell.PATH
                # Also carve the cell between current and neighbor
                self.grid[(current[0] + nx) // 2][(current[1] + ny) // 2] = Cell.PATH
                stack.append((nx, ny))

    def get_unvisited_neighbors(self, x: int, y: int) -> List[Tuple[int, int]]:
        neighbors = []
        for dx, dy in [(0, 2), (2, 0), (0, -2), (-2, 0)]:
            nx, ny = x + dx, y + dy
            if (0 <= nx < self.size and 0 <= ny < self.size and 
                self.grid[nx][ny] == Cell.WALL):
                neighbors.append((nx, ny))
        return neighbors

    def is_solvable(self) -> bool:
        visited = set()
        stack = [self.start]
        visited.add(self.start)

        while stack:
            x, y = stack.pop()
            if (x, y) == self.goal:
                return True

            for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                nx, ny = x + dx, y + dy
                if (0 <= nx < self.size and 0 <= ny < self.size and 
                    self.grid[nx][ny] != Cell.WALL and 
                    (nx, ny) not in visited):
                    stack.append((nx, ny))
                    visited.add((nx, ny))
        return False

    def display(self):
        # Clear screen and move to top
        os.system('clear' if os.name == 'posix' else 'cls')
        
        for row in self.grid:
            for cell in row:
                if isinstance(cell, Cell):
                    cell = cell.value
                color = {
                    '#': '\033[94m█\033[0m',      # Blue
                    ' ': ' ',
                    '•': '\033[92m•\033[0m',  # Green
                    '@': '\033[91m@\033[0m',   # Red
                    '·': '\033[93m·\033[0m',   # Yellow
                }[cell]
                print(color, end='')
            print()
        sys.stdout.flush()

def manhattan_distance(pos: Tuple[int, int], goal: Tuple[int, int]) -> int:
    return abs(pos[0] - goal[0]) + abs(pos[1] - goal[1])

def ida_star(maze: Maze) -> Optional[List[Tuple[int, int]]]:
    def search(path: List[Tuple[int, int]], g: int, bound: int, visited: Set[Tuple[int, int]]):
        current = path[-1]
        f = g + manhattan_distance(current, maze.goal)
        
        if f > bound:
            return f, None
            
        if current == maze.goal:
            return True, path.copy()
            
        min_bound = float('inf')
        visited.add(current)
        maze.grid[current[0]][current[1]] = Cell.VISITED
        
        for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
            nx, ny = current[0] + dx, current[1] + dy
            next_pos = (nx, ny)
            
            if (0 <= nx < maze.size and 0 <= ny < maze.size and 
                maze.grid[nx][ny] != Cell.WALL and 
                next_pos not in path):
                
                path.append(next_pos)
                if isinstance(maze.grid[nx][ny], Cell):
                    prev_cell = maze.grid[nx][ny]
                else:
                    prev_cell = Cell(maze.grid[nx][ny])
                maze.grid[nx][ny] = Cell.CURRENT
                maze.display()
                time.sleep(0.05)
                maze.grid[nx][ny] = prev_cell
                
                result = search(path, g + 1, bound, visited)
                if isinstance(result[0], bool):
                    return result
                min_bound = min(min_bound, result[0])
                path.pop()
                
        return min_bound, None

    bound = manhattan_distance(maze.start, maze.goal)
    path = [maze.start]
    visited = set()

    while True:
        result = search(path, 0, bound, visited)
        if isinstance(result[0], bool):
            return result[1]
        bound = result[0]
        if bound == float('inf'):
            return None
        path = [maze.start]  # Reset path but keep visited set

def main():
    size = int(sys.argv[1]) if len(sys.argv) > 1 else 15
    size = max(5, min(50, size))
    
    print(f"Creating maze of size {size}", file=sys.stderr)
    maze = Maze(size)
    
    while True:
        print("Generating maze...", file=sys.stderr)
        maze.generate()
        if maze.is_solvable():
            print("Found solvable maze", file=sys.stderr)
            break
        maze = Maze(size)  # Reset maze if not solvable

    print("Starting solve...", file=sys.stderr)
    maze.display()

    solution = ida_star(maze)
    if solution:
        print(f"\nSolution found! Path length: {len(solution)}")
        for x, y in solution:
            maze.grid[x][y] = Cell.SOLUTION
        maze.display()
        time.sleep(2)
    else:
        print("No solution found!")

if __name__ == "__main__":
    main()
