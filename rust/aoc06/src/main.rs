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

fn part1(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                start = (i, j);
                break;
            }
        }
    }

    // println!("Start: {:?}", start);

    let locs = simulate(start, &grid);

    println!("{:?}", locs.len());
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut start = (0, 0);
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '^' {
                start = (i, j);
                break;
            }
        }
    }

    let mut locs = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                grid[r][c] = '#';
                if simulateLoop(start, &grid) {
                    // println!("Loop found at: {}, {}", r, c);
                    locs.insert((r, c));
                }
                grid[r][c] = '.';
            }
        }
    }

    println!("{:?}", locs.len());

    Ok(())
}

fn simulate(start: (usize, usize), grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let mut dir: (i32, i32) = (-1, 0);

    let mut locs: HashSet<(usize, usize)> = HashSet::new();
    locs.insert(start);

    let mut current: (i32, i32) = (start.0 as i32, start.1 as i32);
    loop {
        current = (current.0 + dir.0, current.1 + dir.1);

        // check if out of grid bounds
        if current.0 < 0
            || current.0 >= grid.len() as i32
            || current.1 >= grid[0].len() as i32
            || current.1 < 0
        {
            break;
        }

        if grid[current.0 as usize][current.1 as usize] == '#' {
            // backup
            current = (current.0 - dir.0, current.1 - dir.1);

            // turn right
            dir = match dir {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => panic!("invalid direction"),
            };
        }

        locs.insert((current.0 as usize, current.1 as usize));
    }

    locs
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn dir(&self) -> (i32, i32) {
        match *self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn simulateLoop(start: (usize, usize), grid: &[Vec<char>]) -> bool {
    let mut dir = Direction::Up;

    // let mut locs: HashSet<(usize, usize)> = HashSet::new();
    // locs.insert((start.0, start.1, Direction::Up));

    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();
    visited.insert((start.0, start.1, Direction::Up));

    let mut current: (i32, i32) = (start.0 as i32, start.1 as i32);
    loop {
        current = (current.0 + dir.dir().0, current.1 + dir.dir().1);

        if visited.contains(&(current.0 as usize, current.1 as usize, dir)) {
            return true;
        }

        // check if out of grid bounds
        if current.0 < 0
            || current.0 >= grid.len() as i32
            || current.1 >= grid[0].len() as i32
            || current.1 < 0
        {
            break;
        }

        if grid[current.0 as usize][current.1 as usize] == '#' {
            // backup
            current = (current.0 - dir.dir().0, current.1 - dir.dir().1);

            // turn right
            dir = dir.rotate();
        }

        visited.insert((current.0 as usize, current.1 as usize, dir));
        // locs.insert((current.0 as usize, current.1 as usize));
    }

    // locs
    false
}
