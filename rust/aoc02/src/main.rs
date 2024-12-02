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
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;
    for row in data {
        let is_safe = is_safe(&row);
        if is_safe {
            count += 1;
        }
    }

    println!("{}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let data: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    let mut count = 0;
    for row in data {
        let safe = is_safe(&row);
        if safe {
            count += 1;
        } else {
            for i in 0..row.len() {
                let mut row_copy = row.clone();
                row_copy.remove(i);
                let is_safe = is_safe(&row_copy);
                if is_safe {
                    count += 1;
                    break;
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn is_safe(row: &[i32]) -> bool {
    let is_increasing = row.windows(2).all(|pair| pair[0] < pair[1]);
    let is_decreasing = row.windows(2).all(|pair| pair[0] > pair[1]);
    let diffs: Vec<i32> = row
        .windows(2)
        .map(|pair| (pair[0] - pair[1]).abs())
        .collect();
    let in_range = diffs.iter().all(|diff| *diff >= 1 && *diff <= 3);
    let mut is_safe = false;
    if (is_increasing || is_decreasing) && (in_range) {
        is_safe = true;
    }

    // println!(
    //     "{:?} increasing: {}, decreasing: {}, diffs: {:?}, in_range: {} -> {is_safe}",
    //     row, is_increasing, is_decreasing, diffs, in_range
    // );

    is_safe
}
