use std::collections::{HashMap, HashSet, VecDeque};
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

    // let mut sum = 0;
    // for page in invalid_pages {
    //     let mid = page.len() / 2;
    //     sum += page[mid];
    // }

    let mut corrected = vec![];
    for page in &invalid_pages {
        corrected.push(correct_order(&before_map, page));
    }

    let sum_part2: usize = corrected
        .iter()
        .map(|u| {
            let mid = u.len() / 2;
            u[mid]
        })
        .sum();

    println!(
        "Part 2 sum of middle pages (from corrected updates): {}",
        sum_part2
    );

    Ok(())
}

// Produce a correct order for the given update by topological sorting
fn correct_order(adjacency: &HashMap<usize, HashSet<usize>>, update: &[usize]) -> Vec<usize> {
    let set: HashSet<_> = update.iter().cloned().collect();

    // Build a subgraph for these pages
    let mut sub_adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    for &p in &set {
        in_degree.insert(p, 0);
        // Initialize empty adjacency sets so that all pages appear, even if no edges
        sub_adjacency.entry(p).or_default();
    }

    // Add edges only for pages in this update
    for (&x, ys) in adjacency {
        if set.contains(&x) {
            for &y in ys {
                if set.contains(&y) {
                    // Edge x -> y
                    sub_adjacency.entry(x).or_default().insert(y);
                }
            }
        }
    }

    // Compute in-degrees
    for (&x, ys) in &sub_adjacency {
        for &y in ys {
            *in_degree.get_mut(&y).unwrap() += 1;
        }
    }

    // Topological sort (Kahn's Algorithm)
    let mut q = VecDeque::new();
    for (&node, &deg) in &in_degree {
        if deg == 0 {
            q.push_back(node);
        }
    }

    let mut result = Vec::new();
    while let Some(node) = q.pop_front() {
        result.push(node);
        if let Some(children) = sub_adjacency.get(&node) {
            for &c in children {
                let d = in_degree.get_mut(&c).unwrap();
                *d -= 1;
                if *d == 0 {
                    q.push_back(c);
                }
            }
        }
    }

    // result now contains a valid topological order
    // (assuming a unique valid ordering exists)
    result
}
