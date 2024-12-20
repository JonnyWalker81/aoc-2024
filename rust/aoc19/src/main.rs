use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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
    let (a, d) = input.trim().split_once("\n\n").unwrap();

    let available: HashSet<&str> = a.trim().split(", ").collect();
    let designs: Vec<&str> = d.trim().split("\n").collect();

    let mut count = 0;
    for d in designs {
        let mut dp = vec![false; d.len() + 1];
        dp[0] = true;

        if is_possible(&available, d, &mut dp) {
            // println!("{} is possible", d);
            count += 1;
        }
    }

    println!("count: {}", count);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (a, d) = input.trim().split_once("\n\n").unwrap();

    let available: HashSet<&str> = a.trim().split(", ").collect();
    let designs: Vec<&str> = d.trim().split("\n").collect();

    let mut count = 0;
    for d in designs {
        let mut dp = vec![0; d.len() + 1];
        dp[0] = 1;

        count += all_possible(&available, d, &mut dp);
    }

    println!("count: {}", count);

    Ok(())
}

fn is_possible(available: &HashSet<&str>, design: &str, dp: &mut Vec<bool>) -> bool {
    for i in 1..=design.len() {
        for j in 0..i {
            if dp[j] && available.contains(&design[j..i]) {
                dp[i] = true;
                break;
            }
        }
    }

    dp[design.len()]
}

fn all_possible(available: &HashSet<&str>, design: &str, dp: &mut Vec<i64>) -> i64 {
    for i in 1..=design.len() {
        for j in 0..i {
            if dp[j] > 0 && available.contains(&design[j..i]) {
                dp[i] += dp[j];
            }
        }
    }

    dp[design.len()]
}
