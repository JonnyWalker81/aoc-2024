use std::collections::{HashSet, VecDeque};
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
    let bytes: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            let c = iter.next().unwrap().parse::<i32>().unwrap();
            let r = iter.next().unwrap().parse::<i32>().unwrap();
            (r, c)
        })
        .collect();

    let byte_set = bytes
        .iter()
        .take(1024)
        .cloned()
        .collect::<std::collections::HashSet<_>>();

    // for b in bytes.iter().take(1024) {
    //     println!("{:?}", b);
    // }

    // print_grid(&byte_set, 70, 70);

    let score = dfs(&byte_set, 0, 0, 70, 70);

    println!("{}", score);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let bytes: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(",");
            let r = iter.next().unwrap().parse::<i32>().unwrap();
            let c = iter.next().unwrap().parse::<i32>().unwrap();
            (r, c)
        })
        .collect();

    let mut low = 0;
    let mut high = bytes.len();
    let mut blocking_index = None;

    while low < high {
        let mid = (low + high) / 2;

        // Construct the grid with the first `mid` bytes
        let byte_set = bytes
            .iter()
            .take(mid)
            .cloned()
            .collect::<std::collections::HashSet<_>>();

        let score = bfs(&byte_set, 0, 0, 70, 70);

        if score == -1 {
            // Path is blocked at `mid`, try to see if there's an earlier block
            blocking_index = Some(mid);
            high = mid;
        } else {
            // Path is still open, try more bytes
            low = mid + 1;
        }
    }

    // At this point, `blocking_index` holds the earliest byte index that blocks the path.
    // Print the coordinates of that byte.
    if let Some(idx) = blocking_index {
        println!("{}: {},{}", idx, bytes[idx - 1].0, bytes[idx - 1].1);
    } else {
        // If somehow no blocking byte was found (shouldn't happen in the puzzle),
        // you can handle that case here.
        println!("No blocking byte found");
    }

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct State {
    pos: (i32, i32),
    score: i32,
}

fn dfs(grid: &HashSet<(i32, i32)>, r: i32, c: i32, width: usize, height: usize) -> i32 {
    let mut visited = HashSet::new();
    let state = State {
        pos: (r, c),
        score: 0,
    };

    // let mut stack = BinaryHeap::new();
    let mut stack = vec![state];
    // stack.push(Reverse(state));

    while let Some(state) = min_score(&mut stack) {
        let (r, c) = state.pos;

        if r == width as i32 && c == height as i32 {
            return state.score;
        }

        if visited.contains(&(r, c)) {
            continue;
        }

        visited.insert((r, c));

        for (dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_r = r + dr;
            let new_c = c + dc;

            if new_r < 0 || new_r > height as i32 || new_c < 0 || new_c > width as i32 {
                continue;
            }

            if !visited.contains(&(new_r, new_c)) && !grid.contains(&(new_r, new_c)) {
                // println!("pushing: {} {}", new_r, new_c);
                stack.push(State {
                    pos: (new_r, new_c),
                    score: state.score + 1,
                });
            }
        }
    }

    -1
}

fn bfs(grid: &HashSet<(i32, i32)>, r: i32, c: i32, width: usize, height: usize) -> i32 {
    let mut visited = HashSet::new();
    let state = State {
        pos: (r, c),
        score: 0,
    };

    // let mut stack = BinaryHeap::new();
    let mut stack = VecDeque::new();
    stack.push_back(state);

    while let Some(state) = stack.pop_front() {
        let (r, c) = state.pos;

        if r == width as i32 && c == height as i32 {
            return state.score;
        }

        if visited.contains(&(r, c)) {
            continue;
        }

        visited.insert((r, c));

        for (dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_r = r + dr;
            let new_c = c + dc;

            if new_r < 0 || new_r > height as i32 || new_c < 0 || new_c > width as i32 {
                continue;
            }

            if !visited.contains(&(new_r, new_c)) && !grid.contains(&(new_r, new_c)) {
                // println!("pushing: {} {}", new_r, new_c);
                stack.push_back(State {
                    pos: (new_r, new_c),
                    score: state.score + 1,
                });
            }
        }
    }

    -1
}

fn min_score(stack: &mut Vec<State>) -> Option<State> {
    if stack.is_empty() {
        return None;
    }

    let mut min = i32::MAX;
    let mut index = usize::MAX;
    for (i, s) in stack.iter().enumerate() {
        if s.score < min {
            min = s.score;
            index = i;
        }
    }

    Some(stack.remove(index))
}

fn print_grid(grid: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for r in 0..height {
        for c in 0..width {
            if grid.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
