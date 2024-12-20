use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn rotate_clockwise(&self) -> Self {
        Point::new(self.y, -self.x)
    }

    fn rotate_counter_clockwise(&self) -> Self {
        Point::new(-self.y, self.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty(Point),
    Wall(Point),
    Start(Point),
    End(Point),
}

impl Tile {
    fn point(&self) -> Point {
        match self {
            Tile::Empty(p) => *p,
            Tile::Wall(p) => *p,
            Tile::Start(p) => *p,
            Tile::End(p) => *p,
        }
    }
}

fn part1(input: &str) -> Result<()> {
    let m: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut start = Tile::Empty(Point::new(0, 0));
    let mut end = Tile::Empty(Point::new(0, 0));

    let mut maze = vec![vec![Tile::Empty(Point::new(0, 0)); m[0].len()]; m.len()];

    for row in 0..m.len() {
        for col in 0..m[row].len() {
            let p = Point::new(row as i32, col as i32);
            if m[row][col] == 'S' {
                start = Tile::Start(p);
                maze[row][col] = start;
            } else if m[row][col] == 'E' {
                end = Tile::End(p);
                maze[row][col] = end;
            } else if m[row][col] == '#' {
                maze[row][col] = Tile::Wall(p);
            } else if m[row][col] == '.' {
                maze[row][col] = Tile::Empty(p);
            }
        }
    }

    println!("{:?}", start);
    println!("{:?}", end);

    let score = dfs(&maze, start.point(), end.point());
    print_maze(&maze);

    println!("{}", score);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let m: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut start = Tile::Empty(Point::new(0, 0));
    let mut end = Tile::Empty(Point::new(0, 0));

    let mut maze = vec![vec![Tile::Empty(Point::new(0, 0)); m[0].len()]; m.len()];

    for row in 0..m.len() {
        for col in 0..m[row].len() {
            let p = Point::new(row as i32, col as i32);
            if m[row][col] == 'S' {
                start = Tile::Start(p);
                maze[row][col] = start;
            } else if m[row][col] == 'E' {
                end = Tile::End(p);
                maze[row][col] = end;
            } else if m[row][col] == '#' {
                maze[row][col] = Tile::Wall(p);
            } else if m[row][col] == '.' {
                maze[row][col] = Tile::Empty(p);
            }
        }
    }

    println!("{:?}", start);
    println!("{:?}", end);

    // let paths = bfs_find_all(&maze, start.point(), end.point());
    // let paths = dfs_all(&maze, start.point(), end.point());
    // print_maze(&maze);

    // println!("{}", score);
    // println!("{:?}", paths.len());
    // let tiles: HashSet<Point> = paths.iter().flat_map(|p| p.iter()).cloned().collect();
    // println!("{:?}", tiles.len());

    // print_maze_path(&maze, tiles);

    let start_state = State {
        pos: start.point(),
        dir: Direction::East,
        score: 0,
        path: vec![],
    };

    let Some((paths, _)) = pathfinding::prelude::astar_bag_collect(
        &start_state,
        |state| {
            let neighbors = vec![
                (
                    State::new(
                        Point::new(
                            state.pos.x + state.dir.to_point().x,
                            state.pos.y + state.dir.to_point().y,
                        ),
                        state.dir,
                        1,
                    ),
                    1,
                ),
                (
                    State::new(state.pos, state.dir.rotate_clockwise(), 1001),
                    1001,
                ),
                (
                    State::new(state.pos, state.dir.rotate_counter_clockwise(), 1001),
                    1001,
                ),
            ];

            let mut successors = vec![];
            for (i, n) in neighbors.iter().enumerate() {
                if n.0.pos.x < 0
                    || n.0.pos.y < 0
                    || n.0.pos.x >= maze.len() as i32
                    || n.0.pos.y >= maze[0].len() as i32
                {
                    continue;
                }

                match maze[n.0.pos.x as usize][n.0.pos.y as usize] {
                    Tile::Empty(_) | Tile::Start(_) | Tile::End(_) => {
                        successors.push((n.0.clone(), n.1));
                    }
                    Tile::Wall(_) => {
                        continue;
                    }
                }
            }

            successors
        },
        |state| (state.pos.x - end.point().x).abs() + (state.pos.y - end.point().y).abs(),
        |state| state.pos == end.point(),
    ) else {
        panic!("No path found");
    };

    let tiles: HashSet<Point> = paths.iter().flat_map(|p| p.iter().map(|s| s.pos)).collect();
    print_maze_path(&maze, &tiles);
    println!("{:?}", tiles.len());

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    dir: Direction,
    score: i32,
    path: Vec<Point>,
}

impl State {
    fn new(pos: Point, dir: Direction, score: i32) -> Self {
        Self {
            pos,
            dir,
            score,
            path: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn to_point(&self) -> Point {
        match self {
            Direction::North => Point::new(-1, 0),
            Direction::South => Point::new(1, 0),
            Direction::East => Point::new(0, 1),
            Direction::West => Point::new(0, -1),
        }
    }
}

fn dfs(maze: &[Vec<Tile>], start: Point, end: Point) -> i32 {
    let mut visited = HashSet::new();

    let state = State {
        pos: start,
        dir: Direction::East,
        score: 0,
        path: vec![],
    };

    let mut stack = vec![state];
    while let Some(state) = min_score(&mut stack) {
        if state.pos == end {
            println!("found end: {:?}", state);
            return state.score;
        }

        if visited.contains(&(state.pos, state.dir)) {
            continue;
        }

        visited.insert((state.pos, state.dir));

        let neighbors = vec![
            (
                Point::new(
                    state.pos.x + state.dir.to_point().x,
                    state.pos.y + state.dir.to_point().y,
                ),
                state.dir,
                1,
            ),
            (state.pos, state.dir.rotate_clockwise(), 1000),
            (state.pos, state.dir.rotate_counter_clockwise(), 1000),
        ];

        for (i, n) in neighbors.iter().enumerate() {
            if visited.contains(&(n.0, n.1)) {
                continue;
            }

            if n.0.x < 0 || n.0.y < 0 || n.0.x >= maze.len() as i32 || n.0.y >= maze[0].len() as i32
            {
                continue;
            }

            match maze[n.0.x as usize][n.0.y as usize] {
                Tile::Empty(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                        path: state.path.clone(),
                    });
                }
                Tile::Wall(_) => {
                    continue;
                }
                Tile::Start(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                        path: state.path.clone(),
                    });
                }
                Tile::End(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                        path: state.path.clone(),
                    });
                }
            }
        }
    }

    0
}

fn dfs_all(maze: &[Vec<Tile>], start: Point, end: Point) -> Vec<Vec<Point>> {
    let mut visited = HashSet::new();

    let state = State {
        pos: start,
        dir: Direction::East,
        score: 0,
        path: vec![start],
    };

    let mut paths = vec![];

    let mut stack = vec![state];
    while let Some(mut state) = min_score(&mut stack) {
        if state.pos == end {
            println!("found end: {:?}", state);
            paths.push(state.path.clone());
        }

        if visited.contains(&(state.pos, state.dir)) {
            continue;
        }

        visited.insert((state.pos, state.dir));

        let neighbors = vec![
            (
                Point::new(
                    state.pos.x + state.dir.to_point().x,
                    state.pos.y + state.dir.to_point().y,
                ),
                state.dir,
                1,
            ),
            (state.pos, state.dir.rotate_clockwise(), 1000),
            (state.pos, state.dir.rotate_counter_clockwise(), 1000),
        ];

        for (i, n) in neighbors.iter().enumerate() {
            if visited.contains(&(n.0, n.1)) {
                continue;
            }

            if n.0.x < 0 || n.0.y < 0 || n.0.x >= maze.len() as i32 || n.0.y >= maze[0].len() as i32
            {
                continue;
            }

            state.path.push(n.0);

            match maze[n.0.x as usize][n.0.y as usize] {
                Tile::Empty(_) | Tile::Start(_) | Tile::End(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                        path: state.path.clone(),
                    });
                }
                Tile::Wall(_) => {
                    continue;
                }
            }
        }
    }

    paths
}

// function find_all_paths_bfs(maze, start, end):
//     queue = [(start, [start])]  # Queue of (current_position, current_path)
//     paths = []

//     while queue is not empty:
//         (current_position, current_path) = queue.pop(0)

//         if current_position == end:
//             # Found a valid path
//             paths.append(current_path)
//             continue

//         # Explore all valid neighbors
//         for neighbor in get_neighbors(current_position, maze):
//             if neighbor not in current_path:  # Avoid cycles
//                 queue.append((neighbor, current_path + [neighbor]))

//     return paths

fn bfs_find_all(maze: &[Vec<Tile>], start: Point, end: Point) -> Vec<HashSet<Point>> {
    let mut visited = HashSet::new();
    let mut paths = vec![];

    let state = State {
        pos: start,
        dir: Direction::East,
        score: 0,
        path: vec![start], // Use Vec to track the current path
    };

    let mut stack = VecDeque::new();
    stack.push_back(state);

    while let Some(mut state) = stack.pop_front() {
        // If we reach the end, save the path
        if state.pos == end {
            println!("found end: {:?}", state.path);
            paths.push(state.path.iter().cloned().collect()); // Convert Vec to HashSet for uniqueness
            continue;
        }

        // Avoid revisiting the same state (position + direction)
        if visited.contains(&(state.pos, state.dir)) {
            continue;
        }
        visited.insert((state.pos, state.dir));

        // Generate neighbors
        let neighbors = vec![
            (
                Point::new(
                    state.pos.x + state.dir.to_point().x,
                    state.pos.y + state.dir.to_point().y,
                ),
                state.dir,
                1,
            ),
            (state.pos, state.dir.rotate_clockwise(), 1000),
            (state.pos, state.dir.rotate_counter_clockwise(), 1000),
        ];

        for (n_pos, n_dir, cost) in neighbors {
            // Skip already visited states
            if visited.contains(&(n_pos, n_dir)) {
                continue;
            }

            // Skip out-of-bounds tiles
            if n_pos.x < 0
                || n_pos.y < 0
                || n_pos.x >= maze.len() as i32
                || n_pos.y >= maze[0].len() as i32
            {
                continue;
            }

            match maze[n_pos.x as usize][n_pos.y as usize] {
                Tile::Empty(_) | Tile::Start(_) | Tile::End(_) => {
                    let mut new_path = state.path.clone();
                    new_path.push(n_pos); // Add the new position to the path
                    stack.push_back(State {
                        pos: n_pos,
                        dir: n_dir,
                        score: state.score + cost,
                        path: new_path,
                    });
                }
                Tile::Wall(_) => {
                    continue;
                }
            }
        }
    }

    paths
}

fn print_maze(maze: &Vec<Vec<Tile>>) {
    for row in maze {
        for col in row {
            match col {
                Tile::Empty(_) => print!("."),
                Tile::Wall(_) => print!("#"),
                Tile::Start(_) => print!("S"),
                Tile::End(_) => print!("E"),
            }
        }
        println!();
    }
}

fn print_maze_path(maze: &Vec<Vec<Tile>>, path: &HashSet<Point>) {
    for (r, row) in maze.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if path.contains(&Point::new(r as i32, c as i32)) {
                print!("O");
            } else {
                match col {
                    Tile::Empty(_) => print!("."),
                    Tile::Wall(_) => print!("#"),
                    Tile::Start(_) => {}
                    Tile::End(_) => {}
                }
            }
        }
        println!();
    }
}

fn min_score(stack: &mut Vec<State>) -> Option<State> {
    if stack.is_empty() {
        return None;
    }

    let mut min = i32::MAX;
    let mut index = usize::MAX;
    for (i, s) in stack.iter().enumerate() {
        if s.score < min {
            min = s.score;
            index = i;
        }
    }

    Some(stack.remove(index))
}
