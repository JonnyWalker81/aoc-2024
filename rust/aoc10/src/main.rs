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
    let mut trailheads: HashSet<(i64, i64)> = HashSet::new();
    let grid: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                trailheads.insert((r as i64, c as i64));
            }
        }
    }

    let mut sum = 0;
    for (r, c) in &trailheads {
        // println!("{}, {}", x, y);
        let found = dfs(&grid, *r, *c);
        // println!("found trails: {}", found.len());
        sum += found.len();
    }

    println!("sum: {}", sum);

    Ok(())
}

fn dfs(grid: &Vec<Vec<i64>>, r: i64, c: i64) -> HashSet<(i64, i64)> {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut stack: Vec<(i64, i64)> = Vec::new();

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    stack.push((r, c));

    let mut found = HashSet::new();
    while let Some((r, c)) = stack.pop() {
        if visited.contains(&(r, c)) {
            continue;
        }

        visited.insert((r, c));

        if grid[r as usize][c as usize] == 9 {
            // println!("Found trailhead at {}, {}", x, y);
            found.insert((r, c));
        }

        for dir in &dirs {
            let (dr, dc) = dir;
            let nr = r + dr;
            let nc = c + dc;

            if nc < 0 || nc >= grid[0].len() as i64 || nr < 0 || nr >= grid.len() as i64 {
                continue;
            }

            if grid[nr as usize][nc as usize] == grid[r as usize][c as usize] + 1 {
                stack.push((nr, nc));
            }
        }
    }

    found
}

fn part2(input: &str) -> Result<()> {
    let mut trailheads: HashSet<(i64, i64)> = HashSet::new();
    let grid: Vec<Vec<i64>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                trailheads.insert((r as i64, c as i64));
            }
        }
    }

    let mut memo: Vec<Vec<i64>> = vec![vec![0; grid[0].len()]; grid.len()];

    let mut sum = 0;
    for (r, c) in &trailheads {
        let count = count_trails(&grid, *r, *c, &mut memo);
        // println!("count: {}", count);
        sum += count;
    }

    println!("sum: {}", sum);

    Ok(())
}

fn count_trails(grid: &Vec<Vec<i64>>, r: i64, c: i64, memo: &mut Vec<Vec<i64>>) -> i64 {
    if memo[r as usize][c as usize] != 0 {
        return memo[r as usize][c as usize];
    }

    if grid[r as usize][c as usize] == 9 {
        memo[r as usize][c as usize] = 1;
        return 1;
    }

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut count = 0;
    for dir in &dirs {
        let (dr, dc) = dir;
        let nr = r + dr;
        let nc = c + dc;

        if nc < 0 || nc >= grid[0].len() as i64 || nr < 0 || nr >= grid.len() as i64 {
            continue;
        }

        if grid[nr as usize][nc as usize] == grid[r as usize][c as usize] + 1 {
            count += count_trails(grid, nr, nc, memo);
        }
    }

    memo[r as usize][c as usize] = count;
    count
}
