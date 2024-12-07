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

#[derive(Debug)]
struct Equation {
    test_value: i64,
    vals: Vec<i64>,
}

fn part1(input: &str) -> Result<()> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let vals = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            Equation { test_value, vals }
        })
        .collect();

    let mut sum = 0;
    for eq in equations {
        let can_achieve = can_achieve_target(eq.test_value, &eq.vals);
        if can_achieve {
            sum += eq.test_value;
        }
    }

    println!("sum: {}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let vals = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|val| val.parse().unwrap())
                .collect();
            Equation { test_value, vals }
        })
        .collect();

    let mut sum = 0;
    for eq in equations {
        let can_achieve = can_achieve_target_dfs(eq.test_value, &eq.vals);
        if can_achieve {
            sum += eq.test_value;
        }
    }

    println!("sum: {}", sum);

    Ok(())
}

fn can_achieve_target(target: i64, vals: &[i64]) -> bool {
    let mut dp: Vec<HashSet<i64>> = vec![HashSet::new(); vals.len()];

    dp[0].insert(vals[0]);

    for i in 1..vals.len() {
        let current = vals[i];
        for j in dp[i - 1].iter().cloned().collect::<Vec<i64>>() {
            dp[i].insert(j + current);
            dp[i].insert(j * current);
        }
    }

    for set in dp.iter() {
        if set.contains(&target) {
            return true;
        }
    }

    false
}

fn can_achieve_target_dfs(target: i64, vals: &[i64]) -> bool {
    let mut memo: HashMap<(i64, i64), bool> = HashMap::new();

    dfs(1, vals[0], vals, target, &mut memo)
}

fn dfs(
    current_index: i64,
    current_value: i64,
    vals: &[i64],
    target: i64,
    memo: &mut HashMap<(i64, i64), bool>,
) -> bool {
    if current_index as usize == vals.len() {
        return current_value == target;
    }

    let state_key = (current_index, current_value);
    if let Some(&result) = memo.get(&state_key) {
        return result;
    }

    let next_num = vals[current_index as usize];

    let mut can_form_target = false;

    let new_current = current_value + next_num;
    if dfs(current_index + 1, new_current, vals, target, memo) {
        can_form_target = true;
    }

    let mul = current_value * next_num;
    if dfs(current_index + 1, mul, vals, target, memo) {
        can_form_target = true;
    }

    let concatenated_operand = concatenate_operands(current_value, next_num);
    if dfs(current_index + 1, concatenated_operand, vals, target, memo) {
        can_form_target = true;
    }

    memo.insert(state_key, can_form_target);
    can_form_target
}

fn concatenate_operands(a: i64, b: i64) -> i64 {
    let a_str = a.to_string();
    let b_str = b.to_string();
    let concat = format!("{}{}", a_str, b_str);
    concat.parse().unwrap()
}
