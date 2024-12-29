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
    let mut nodes = HashMap::new();
    // let mut comps = HashSet::new();
    for line in input.trim().lines() {
        let (from, to) = line.split_once("-").unwrap();
        nodes.entry(from).or_insert_with(Vec::new).push(to);
        nodes.entry(to).or_insert_with(Vec::new).push(from);
        // comps.insert(from);
        // comps.insert(to);
    }

    // for (from, tos) in nodes.iter() {
    //     println!("{} -> {:?}", from, tos);
    // }

    let sorted_keys = {
        let mut keys: Vec<_> = nodes.keys().collect();
        keys.sort();
        keys
    };

    let mut triangles = vec![];
    for (i, a) in sorted_keys.iter().enumerate() {
        let sorted_neighbors = {
            let mut neighbors = nodes.get(*a).unwrap().clone();
            neighbors.sort();
            neighbors
        };

        for b in sorted_neighbors {
            if b <= **a {
                continue;
            }

            let common_neighbors = {
                let mut common = Vec::new();
                for neighbor in nodes.get(*a).unwrap() {
                    if nodes.get(b).unwrap().contains(neighbor) {
                        common.push(neighbor);
                    }
                }
                common.sort();
                common
            };

            for c in common_neighbors {
                if *c <= b {
                    continue;
                }

                // println!("{}-{}-{}", a, b, c);
                triangles.push((a, b, c));
            }
        }
    }

    // println!("Triangles: {:?}", triangles);
    println!("Triangles: {}", triangles.len());

    let starts_with_t = triangles
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count();

    println!("Starts with t: {}", starts_with_t);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.trim().lines() {
        if line.trim().is_empty() {
            continue;
        }
        // Split "kh-tc" into ("kh", "tc")
        let parts: Vec<_> = line.trim().split('-').collect();
        if parts.len() == 2 {
            let a = parts[0].to_string();
            let b = parts[1].to_string();

            adjacency
                .entry(a.clone())
                .or_insert_with(HashSet::new)
                .insert(b.clone());
            adjacency.entry(b).or_insert_with(HashSet::new).insert(a);
        }
    }

    // 2. Find the maximum clique
    let max_clique = find_maximum_clique(&adjacency);

    // 3. Sort & join to get the password
    let mut sorted_clique = max_clique.clone();
    sorted_clique.sort();
    let password = sorted_clique.join(",");

    println!("Largest clique: {:?}", max_clique);
    println!("Password: {}", password);
    Ok(())
}

/// Finds a maximum clique in an undirected graph given by `adjacency`.
/// Returns a `Vec<String>` with the node names in the maximum clique.
fn find_maximum_clique(adjacency: &HashMap<String, HashSet<String>>) -> Vec<String> {
    // Collect all nodes
    let all_nodes: Vec<String> = adjacency.keys().cloned().collect();

    // We'll store the largest clique found so far
    let mut best_clique: Vec<String> = Vec::new();

    /// A helper function for backtracking.
    /// - `current_clique`: the clique we have built so far
    /// - `candidates`: the set of nodes that may still join the clique
    fn backtrack_clique(
        adjacency: &HashMap<String, HashSet<String>>,
        current_clique: &mut Vec<String>,
        candidates: &mut HashSet<String>,
        best_clique: &mut Vec<String>,
    ) {
        // If there are no candidates left, we have a maximal clique
        if candidates.is_empty() {
            if current_clique.len() > best_clique.len() {
                *best_clique = current_clique.clone();
            }
            return;
        }

        // Pruning: if even taking all candidates won't exceed the best, stop.
        if current_clique.len() + candidates.len() <= best_clique.len() {
            return;
        }

        // We'll iterate over a snapshot of the candidates
        let candidate_list: Vec<String> = candidates.iter().cloned().collect();
        for node in candidate_list {
            // Form the new set of candidates as intersection
            // of current candidates and neighbors of `node`
            if let Some(neighbors) = adjacency.get(&node) {
                let mut new_candidates = candidates
                    .intersection(neighbors)
                    .cloned()
                    .collect::<HashSet<_>>();

                // Include this node in the clique
                current_clique.push(node.clone());

                // Recurse
                backtrack_clique(adjacency, current_clique, &mut new_candidates, best_clique);

                // Backtrack
                current_clique.pop();

                // Remove this node from candidates so we don't reuse it at this level
                candidates.remove(&node);
            }
        }
    }

    // Start with all nodes as candidates, empty clique
    let mut candidate_set: HashSet<String> = all_nodes.into_iter().collect();
    let mut current_clique: Vec<String> = Vec::new();

    backtrack_clique(
        adjacency,
        &mut current_clique,
        &mut candidate_set,
        &mut best_clique,
    );

    best_clique
}
