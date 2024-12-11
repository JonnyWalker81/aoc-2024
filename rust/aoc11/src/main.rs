use std::collections::HashMap;
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
    let stones: Vec<i64> = input
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace().map(|l| l.parse().unwrap()))
        .collect();

    let mut memo: HashMap<(i64, i64), i64> = HashMap::new();

    let mut sum = 0;
    for stone in stones.iter() {
        let count = count_stones(*stone, 25, &mut memo);
        sum += count;
    }

    println!("{}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let stones: Vec<i64> = input
        .trim()
        .lines()
        .flat_map(|line| line.split_whitespace().map(|l| l.parse().unwrap()))
        .collect();

    let mut memo: HashMap<(i64, i64), i64> = HashMap::new();

    let mut sum = 0;
    for stone in stones.iter() {
        let count = count_stones(*stone, 75, &mut memo);
        sum += count;
    }

    println!("{}", sum);

    Ok(())
}

fn count_stones(stone: i64, blinks_remaining: i64, memo: &mut HashMap<(i64, i64), i64>) -> usize {
    if blinks_remaining == 0 {
        return 1;
    }

    if let Some(count) = memo.get(&(stone, blinks_remaining)) {
        return *count as usize;
    }

    let result;
    if stone == 0 {
        result = count_stones(1, blinks_remaining - 1, memo);
    } else if stone.to_string().len() % 2 == 0 {
        let first_half = &stone.to_string()[0..stone.to_string().len() / 2]
            .parse::<i64>()
            .unwrap();
        let second_half = &stone.to_string()[stone.to_string().len() / 2..]
            .parse::<i64>()
            .unwrap();
        result = count_stones(*first_half, blinks_remaining - 1, memo)
            + count_stones(*second_half, blinks_remaining - 1, memo);
    } else {
        result = count_stones(stone * 2024, blinks_remaining - 1, memo);
    }

    memo.insert((stone, blinks_remaining), result as i64);

    result
}
