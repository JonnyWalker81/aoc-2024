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
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    input.trim().split('\n').for_each(|l| {
        let parts: Vec<&str> = l.split_whitespace().collect();
        left_list.push(parts[0].parse().expect("number"));
        right_list.push(parts[1].parse().expect("number"));
    });

    left_list.sort();
    right_list.sort();

    let mut result = vec![];
    for i in 0..left_list.len() {
        result.push((left_list[i] - right_list[i]).abs());
    }
    let sum: i32 = result.iter().sum();
    println!("{:?}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    input.trim().split('\n').for_each(|l| {
        let parts: Vec<&str> = l.split_whitespace().collect();
        left_list.push(parts[0].parse().expect("number"));
        right_list.push(parts[1].parse().expect("number"));
    });

    let mut counts = HashMap::new();

    for i in right_list {
        counts.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut result = vec![];
    for i in left_list {
        if let Some(v) = counts.get(&i) {
            result.push(*v * i);
        }
    }

    let sum: i32 = result.iter().sum();
    println!("{:?}", sum);

    Ok(())
}
