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
    let (r, p) = input.trim().split_once("\n\n").unwrap();
    // println!("{:?}", r);
    // println!("{:?}", p);

    let rules: Vec<(usize, usize)> = r
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    // println!("{:#?}", rules);

    let mut before_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut after_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (left, right) in rules.iter() {
        // println!("{} -> {}", *left, *right);
        before_map.entry(*left).or_default().insert(*right);
        after_map.entry(*right).or_default().insert(*left);
    }

    let pages: Vec<Vec<usize>> = p
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    // println!("{:?}", pages);

    let mut valid_pages = vec![];
    for page in pages {
        let mut valid = true;
        for n in 0..page.len() {
            for before in 0..n {
                // println!("before: {:?}", before_map.get(&page[before]));
                if let Some(set) = before_map.get(&page[before]) {
                    if !set.contains(&page[n]) {
                        // println!("invalid");
                        valid = false;
                        break;
                    }
                }
            }

            for after in n + 1..page.len() {
                // println!("after: {:?}", after_map.get(&page[after]));
                if let Some(set) = after_map.get(&page[after]) {
                    if !set.contains(&page[n]) {
                        // println!("invalid");
                        valid = false;
                        break;
                    }
                }
            }
        }
        if valid {
            valid_pages.push(page);
        }
    }

    // println!("{:?}", valid_pages);

    let mut sum = 0;
    for page in valid_pages {
        let mid = page.len() / 2;
        sum += page[mid];
    }

    println!("{:?}", sum);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (r, p) = input.trim().split_once("\n\n").unwrap();
    // println!("{:?}", r);
    // println!("{:?}", p);

    let rules: Vec<(usize, usize)> = r
        .lines()
        .map(|l| {
            let (left, right) = l.split_once("|").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect();

    // println!("{:#?}", rules);

    let mut before_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut after_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    for (left, right) in rules.iter() {
        // println!("{} -> {}", *left, *right);
        before_map.entry(*left).or_default().insert(*right);
        after_map.entry(*right).or_default().insert(*left);
    }

    let pages: Vec<Vec<usize>> = p
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    // println!("{:?}", pages);

    let mut invalid_pages = vec![];
    for page in pages {
        let mut valid = true;
        for n in 0..page.len() {
            for before in 0..n {
                // println!("before: {:?}", before_map.get(&page[before]));
                if let Some(set) = before_map.get(&page[before]) {
                    if !set.contains(&page[n]) {
                        // println!("invalid");
                        valid = false;
                        break;
                    }
                }
            }

            for after in n + 1..page.len() {
                // println!("after: {:?}", after_map.get(&page[after]));
                if let Some(set) = after_map.get(&page[after]) {
                    if !set.contains(&page[n]) {
                        // println!("invalid");
                        valid = false;
                        break;
                    }
                }
            }
        }
        if !valid {
            invalid_pages.push(page);
        }
    }

    // println!("{:?}", valid_pages);

    let mut sum = 0;
    for page in invalid_pages {
        let mid = page.len() / 2;
        sum += page[mid];
    }

    println!("{:?}", sum);

    Ok(())
}
