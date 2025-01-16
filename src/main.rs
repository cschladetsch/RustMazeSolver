use rand::Rng;
use std::{io::Write, thread, time::Duration, env};
use termion::{color, style};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
    Solution,
    Current,
    Visited,
}

struct Maze {
    size: usize,
    grid: Vec<Vec<Cell>>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Maze {
    fn new(size: usize) -> Self {
        let mut grid = vec![vec![Cell::Wall; size]; size];
        // Start at top-left, goal at bottom-right
        let start = (1, 1);
        let goal = (size - 2, size - 2);
        grid[start.0][start.1] = Cell::Path;
        grid[goal.0][goal.1] = Cell::Path;
        
        Maze {
            size,
            grid,
            start,
            goal,
        }
    }

    fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        let mut stack = vec![self.start];
        let mut visited = vec![vec![false; self.size]; self.size];
        visited[self.start.0][self.start.1] = true;
        
        // First, generate a full maze using DFS with randomized neighbor selection
        while let Some(&current) = stack.last() {
            let mut neighbors = Vec::new();
            for (dx, dy) in &[(0, 2), (2, 0), (0, -2), (-2, 0)] {
                let nx = (current.0 as isize + dx) as usize;
                let ny = (current.1 as isize + dy) as usize;
                if nx < self.size - 1 && ny < self.size - 1 && !visited[nx][ny] {
                    neighbors.push((nx, ny));
                }
            }

            if neighbors.is_empty() {
                stack.pop();
            } else {
                // Randomly choose next cell
                let (nx, ny) = neighbors[rng.gen_range(0..neighbors.len())];
                self.grid[nx][ny] = Cell::Path;
                self.grid[(current.0 + nx) / 2][(current.1 + ny) / 2] = Cell::Path;
                visited[nx][ny] = true;
                stack.push((nx, ny));
            }
        }

        // Add some random additional connections to create loops and multiple paths
        for _ in 0..self.size {
            let x = rng.gen_range(1..self.size-1);
            let y = rng.gen_range(1..self.size-1);
            if self.grid[x][y] == Cell::Path {
                // Try to break a random wall
                let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                let (dx, dy) = directions[rng.gen_range(0..directions.len())];
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx < self.size - 1 && ny < self.size - 1 && self.grid[nx][ny] == Cell::Wall {
                    self.grid[nx][ny] = Cell::Path;
                }
            }
        }

        // Ensure start and goal are open
        self.grid[self.start.0][self.start.1] = Cell::Path;
        self.grid[self.goal.0][self.goal.1] = Cell::Path;
        
        // Verify maze is solvable using DFS
        if !self.is_solvable() {
            // If not solvable, connect goal to nearest path
            let mut closest_path = (self.goal.0 - 1, self.goal.1);
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 { continue; }
                    let nx = (self.goal.0 as isize + dx) as usize;
                    let ny = (self.goal.1 as isize + dy) as usize;
                    if self.grid[nx][ny] == Cell::Path {
                        closest_path = (nx, ny);
                        break;
                    }
                }
            }
            self.grid[closest_path.0][closest_path.1] = Cell::Path;
        }
    }

    fn is_solvable(&self) -> bool {
        let mut visited = vec![vec![false; self.size]; self.size];
        let mut stack = vec![self.start];
        visited[self.start.0][self.start.1] = true;

        while let Some((x, y)) = stack.pop() {
            if (x, y) == self.goal {
                return true;
            }
            for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx < self.size && ny < self.size && 
                   !visited[nx][ny] && self.grid[nx][ny] == Cell::Path {
                    stack.push((nx, ny));
                    visited[nx][ny] = true;
                }
            }
        }
        false
    }

    fn manhattan_distance(&self, pos: (usize, usize)) -> usize {
        ((pos.0 as isize - self.goal.0 as isize).abs() +
         (pos.1 as isize - self.goal.1 as isize).abs()) as usize
    }

    fn display(&self) {
        print!("\x1B[H");  // Move cursor to top-left
        for row in &self.grid {
            for &cell in row {
                let symbol = match cell {
                    Cell::Wall => format!("{}█{}", color::Fg(color::Blue), style::Reset),
                    Cell::Path => " ".to_string(),
                    Cell::Solution => format!("{}•{}", color::Fg(color::Green), style::Reset),
                    Cell::Current => format!("{}@{}", color::Fg(color::Red), style::Reset),
                    Cell::Visited => format!("{}·{}", color::Fg(color::Yellow), style::Reset),
                };
                print!("{}", symbol);
            }
            println!();
        }
        std::io::stdout().flush().unwrap();
    }
}

#[derive(Clone)]
struct SearchState {
    path: Vec<(usize, usize)>,
    visited: Vec<Vec<bool>>,
}

enum SearchResult {
    Found(Vec<(usize, usize)>),
    NewBound(usize),
}

fn ida_star(maze: &mut Maze) -> Option<Vec<(usize, usize)>> {
    let initial_estimate = maze.manhattan_distance(maze.start);
    let mut bound = initial_estimate * 3;  // Start with a more generous bound
    let mut state = SearchState {
        path: vec![maze.start],
        visited: vec![vec![false; maze.size]; maze.size],
    };

    while bound < maze.size * maze.size {  // Upper limit to prevent infinite loops
        match search(maze, 0, bound, &mut state) {
            SearchResult::Found(solution) => return Some(solution),
            SearchResult::NewBound(new_bound) => {
                if new_bound == usize::MAX {
                    bound += maze.size;  // More aggressive bound increase
                } else {
                    bound = new_bound + maze.size/2;  // Significant increase to reduce iterations
                }
                state.path = vec![maze.start];  // Keep visited cells marked
            }
        }
    }
    None
}

fn search(
    maze: &mut Maze,
    g: usize,
    bound: usize,
    state: &mut SearchState,
) -> SearchResult {
    let current = *state.path.last().unwrap();
    let h = maze.manhattan_distance(current);
    let f = g + h;

    // Show current position
    let is_start = current == maze.start;
    maze.grid[current.0][current.1] = Cell::Current;
    maze.display();
    thread::sleep(Duration::from_millis(20));

    if f > bound {
        // Mark as visited before returning
        if !is_start {
            maze.grid[current.0][current.1] = Cell::Visited;
        }
        return SearchResult::NewBound(f);
    }

    if current == maze.goal {
        // Found path - mark solution
        for &(x, y) in state.path.iter() {
            maze.grid[x][y] = Cell::Solution;
        }
        maze.display();
        return SearchResult::Found(state.path.clone());
    }

    // Mark current cell as visited in state tracking
    state.visited[current.0][current.1] = true;

    // Get all possible moves and sort by estimated total cost
    let mut moves = Vec::new();
    for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let nx = (current.0 as isize + dx) as usize;
        let ny = (current.1 as isize + dy) as usize;
        
        if nx < maze.size && ny < maze.size && 
           maze.grid[nx][ny] != Cell::Wall && 
           !state.visited[nx][ny] {
            let move_h = maze.manhattan_distance((nx, ny));
            let move_g = g + 1;
            let move_f = move_g + move_h;
            moves.push((move_f, (nx, ny)));
        }
    }
    // Sort moves by total estimated cost
    moves.sort_by_key(|&(cost, _)| cost);

    let mut min_bound = usize::MAX;
    for (_, (nx, ny)) in moves {
        if !state.path.contains(&(nx, ny)) {
            state.path.push((nx, ny));
            
            match search(maze, g + 1, bound, state) {
                SearchResult::Found(solution) => return SearchResult::Found(solution),
                SearchResult::NewBound(new_bound) => {
                    min_bound = min_bound.min(new_bound);
                }
            }
            
            state.path.pop();
        }
    }

    // Mark current as visited before returning
    if !is_start {
        maze.grid[current.0][current.1] = Cell::Visited;
    }
    
    SearchResult::NewBound(min_bound)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size = if args.len() > 1 {
        args[1].parse().unwrap_or(15)
    } else {
        15
    };

    let mut maze = Maze::new(size);
    maze.generate();

    // Clear screen and hide cursor
    print!("\x1B[2J\x1B[?25l");
    std::io::stdout().flush().unwrap();

    if let Some(solution) = ida_star(&mut maze) {
        println!("\nSolution found! Path length: {}", solution.len());
        thread::sleep(Duration::from_secs(2));
    } else {
        println!("No solution found!");
    }

    // Show cursor again
    print!("\x1B[?25h");
    std::io::stdout().flush().unwrap();
}
