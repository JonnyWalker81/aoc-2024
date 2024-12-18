use std::collections::HashSet;
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

    let score = dfs_paths(&maze, start.point(), end.point());
    print_maze(&maze);

    println!("{}", score);
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: Point,
    dir: Direction,
    score: i32,
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
                    });
                }
                Tile::End(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                    });
                }
            }
        }
    }

    0
}

fn dfs_paths(maze: &[Vec<Tile>], pos: Point, end: Point, memo: &mut &[Vec<Tile>]) -> i32 {
    if memo[pos.x][pos.y] != Tile::Empty(Point::new(0, 0)) {
        return 0;
    }

    let mut visited = HashSet::new();

    let state = State {
        pos: start,
        dir: Direction::East,
        score: 0,
    };

    let mut stack = vec![state];
    while let Some(state) = stack.pop() {
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
                    });
                }
                Tile::End(_) => {
                    stack.push(State {
                        pos: n.0,
                        dir: n.1,
                        score: state.score + n.2,
                    });
                }
            }
        }
    }

    0
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
