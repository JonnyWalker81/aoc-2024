use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let grids = input.trim().split("\n\n").collect::<Vec<&str>>();

    let mut lock_grids = vec![];
    let mut key_grids = vec![];
    for line in grids.iter() {
        let l: Vec<Vec<char>> = line.lines().map(|l| l.chars().collect()).collect();
        if line.starts_with(".") {
            key_grids.push(l);
        } else {
            lock_grids.push(l);
        }
    }

    let mut locks = vec![];
    for grid in lock_grids {
        let mut l = vec![];
        // println!("{:?}", grid);
        for c in 0..grid[0].len() {
            let mut count = 0;
            for r in 1..grid.len() {
                if grid[r][c] == '#' {
                    count += 1;
                }
            }
            l.push(count);
        }
        locks.push(l);
    }

    let mut keys = vec![];
    for grid in key_grids {
        let mut l = vec![];
        // println!("{:?}", grid);
        for c in 0..grid[0].len() {
            let mut count = 0;
            for r in 0..grid.len() - 1 {
                if grid[r][c] == '#' {
                    count += 1;
                }
            }
            l.push(count);
        }
        keys.push(l);
    }

    // println!("{:?}", locks);
    // println!("{:?}", keys);

    let mut unique_locks = 0;
    for lock in locks {
        for key in &keys {
            // println!("{:?} {:?}", lock, key);
            let mut overlap = false;
            for i in 0..lock.len() {
                if lock[i] + key[i] >= 6 {
                    // println!("Overlap");
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                unique_locks += 1;
            }
        }
    }

    println!("{:?}", unique_locks);

    Ok(())
}
