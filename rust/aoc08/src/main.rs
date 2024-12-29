use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut freq: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_alphanumeric() {
                freq.entry(grid[i][j])
                    .or_default()
                    .push((i as i64, j as i64));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, anntenas) in freq.iter() {
        if anntenas.len() < 2 {
            continue;
        }

        for i in 0..anntenas.len() {
            let a = anntenas[i];
            for j in i + 1..anntenas.len() {
                let b = anntenas[j];

                let (xA, yA) = a;
                let (xB, yB) = b;

                let p_x1 = 2 * xA - xB;
                let p_y1 = 2 * yA - yB;

                let p_x2 = 2 * xB - xA;
                let p_y2 = 2 * yB - yA;

                if in_bounds(&grid, p_x1, p_y1) {
                    antinodes.insert((p_x1, p_y1));
                }

                if in_bounds(&grid, p_x2, p_y2) {
                    antinodes.insert((p_x2, p_y2));
                }
            }
        }
    }

    println!("{:?}", antinodes.len());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut freq: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j].is_alphanumeric() {
                freq.entry(grid[i][j])
                    .or_default()
                    .push((i as i64, j as i64));
            }
        }
    }

    let mut antinodes = HashSet::new();
    for (_, anntenas) in freq.iter() {
        if anntenas.len() < 2 {
            continue;
        }

        for i in 0..anntenas.len() {
            let a = anntenas[i];
            for j in i + 1..anntenas.len() {
                let b = anntenas[j];

                let (x_A, y_A) = a;
                let (x_B, y_B) = b;

                let dx = x_B - x_A;
                let dy = y_B - y_A;

                let step = gcd(dx, dy);
                let dx_step = dx / step;
                let dy_step = dy / step;

                let mut x = x_A;
                let mut y = y_A;
                loop {
                    x += dx_step;
                    y += dy_step;

                    if in_bounds(&grid, x, y) {
                        antinodes.insert((x, y));
                    } else {
                        break;
                    }
                }

                loop {
                    x -= dx_step;
                    y -= dy_step;

                    if in_bounds(&grid, x, y) {
                        antinodes.insert((x, y));
                    } else {
                        break;
                    }
                }
            }
        }
    }

    println!("{:?}", antinodes.len());

    Ok(())
}

fn in_bounds(grid: &[Vec<char>], x: i64, y: i64) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i64 && y < grid[0].len() as i64
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}
