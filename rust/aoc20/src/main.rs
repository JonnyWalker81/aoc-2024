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
    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut start: (i64, i64) = (0, 0);
    let mut end: (i64, i64) = (0, 0);
    let mut walls = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r][c] {
                'S' => start = (r as i64, c as i64),
                'E' => end = (r as i64, c as i64),
                '#' => {
                    walls.insert((r as i64, c as i64));
                }
                _ => (),
            };
        }
    }

    let base_time = bfs(&grid, &walls, start, end);
    // println!("{}", base_time);

    let mut dist_from_start = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let mut dist_to_end = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '#' {
                continue;
            }

            dist_from_start[r][c] = bfs(&grid, &walls, start, (r as i64, c as i64));
            dist_to_end[r][c] = bfs(&grid, &walls, end, (r as i64, c as i64));
        }
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            let savings = check_cheats(
                &grid,
                &walls,
                (r as i64, c as i64),
                &dist_from_start,
                &dist_to_end,
                base_time as i64,
                100,
            );
            if savings > 0 {
                count += savings;
            }
        }
    }

    println!("{}", count);
    // let mut time_groups: HashMap<usize, Vec<usize>> = HashMap::new();
    // for r in 0..grid.len() {
    //     for c in 0..grid[r].len() {
    //         if walls.contains(&(r as i64, c as i64)) {
    //             let first = (r as i64, c as i64);
    //             let second = get_next_wall(&walls, first);
    //             walls.remove(&first);
    //             walls.remove(&second);
    //             let new_time = bfs(&grid, &walls, start, end);
    //             if new_time < base_time {
    //                 let diff = base_time - new_time;
    //                 println!("({}, {}): {} --> {}", r, c, new_time, diff);
    //                 time_groups.entry(diff).or_default().push(new_time);
    //             }
    //             walls.insert(first);
    //             walls.insert(second);
    //         }
    //     }
    // }

    // for (diff, times) in time_groups {
    //     println!("{}: {:?}", diff, times.len());
    // }
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    let mut start: (i64, i64) = (0, 0);
    let mut end: (i64, i64) = (0, 0);
    let mut walls = HashSet::new();
    let mut path = HashSet::new();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            match grid[r][c] {
                'S' => start = (r as i64, c as i64),
                'E' => end = (r as i64, c as i64),
                '#' => {
                    walls.insert((r as i64, c as i64));
                }
                _ => (),
            };

            if grid[r][c] != '#' {
                path.insert((r as i64, c as i64));
            }
        }
    }

    let count = cheats_bfs(&grid, &path, &walls, start, end, 100);

    println!("{}", count);

    Ok(())
}

fn check_cheats_bfs(
    grid: &[Vec<char>],
    walls: &HashSet<(i64, i64)>,
    start: (i64, i64),
    dist_from_start: &[Vec<usize>],
    dist_to_end: &[Vec<usize>],
    base_time: i64,
    diff_limit: i64,
    cheat_limit: i64,
) -> i64 {
    let mut count = 0;
    if dist_from_start[start.0 as usize][start.1 as usize] == usize::MAX
        || dist_to_end[start.0 as usize][start.1 as usize] == usize::MAX
    {
        return -1;
    }

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited = vec![vec![i64::MAX; grid[0].len()]; grid.len()];

    while let Some((pos, time)) = queue.pop_front() {
        if time >= cheat_limit {
            continue;
        }

        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if !is_valid_cell(grid, next) {
                continue;
            }

            let next_steps = time + 1;
            if next_steps < visited[next.0 as usize][next.1 as usize] {
                visited[next.0 as usize][next.1 as usize] = next_steps;
                queue.push_back((next, next_steps));

                if is_valid_track(grid, walls, next) {
                    // Compute time saved
                    let cheat_time = dist_from_start[start.0 as usize][start.1 as usize]
                        + next_steps as usize
                        + dist_to_end[next.0 as usize][next.1 as usize];
                    let time_saved = base_time - cheat_time as i64;
                    if time_saved >= diff_limit {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn check_cheats(
    grid: &[Vec<char>],
    walls: &HashSet<(i64, i64)>,
    start: (i64, i64),
    dist_from_start: &[Vec<usize>],
    dist_to_end: &[Vec<usize>],
    base_time: i64,
    diff_limit: i64,
) -> i64 {
    let mut count = 0;
    if dist_from_start[start.0 as usize][start.1 as usize] == usize::MAX
        || dist_to_end[start.0 as usize][start.1 as usize] == usize::MAX
    {
        return -1;
    }

    for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next = (start.0 + dir.0, start.1 + dir.1);
        if is_valid_cell(grid, next) && is_valid_track(grid, walls, next) {
            let cheat_time = dist_from_start[start.0 as usize][start.1 as usize]
                + 1
                + dist_to_end[next.0 as usize][next.1 as usize];
            let time_saved = base_time - cheat_time as i64;
            if time_saved >= diff_limit {
                // println!("1-step: ({}, {}): {}", start.0, start.1, time_saved);
                count += 1
            }
        }
    }

    for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let inext = (start.0 + dir.0, start.1 + dir.1);

        if !is_valid_cell(grid, inext) {
            continue;
        }

        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let enext = (inext.0 + dir.0, inext.1 + dir.1);
            if is_valid_cell(grid, enext) && is_valid_track(grid, walls, enext) {
                let cheat_time = dist_from_start[start.0 as usize][start.1 as usize]
                    + 2
                    + dist_to_end[enext.0 as usize][enext.1 as usize];
                let time_saved = base_time - cheat_time as i64;
                if time_saved >= diff_limit {
                    // println!("2-step: ({}, {}): {}", start.0, start.1, time_saved);
                    count += 1;
                }
            }
        }
    }

    count
}

fn is_valid_cell(grid: &[Vec<char>], pos: (i64, i64)) -> bool {
    pos.0 >= 0
        && pos.0 < grid.len() as i64
        && pos.1 >= 0
        && pos.1 < grid[pos.0 as usize].len() as i64
}

fn is_valid_track(grid: &[Vec<char>], walls: &HashSet<(i64, i64)>, pos: (i64, i64)) -> bool {
    grid[pos.0 as usize][pos.1 as usize] != '#'
}

fn get_next_wall(walls: &HashSet<(i64, i64)>, start: (i64, i64)) -> (i64, i64) {
    for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let next = (start.0 as i64 + dir.0, start.1 as i64 + dir.1);

        if walls.contains(&next) {
            return (next.0, next.1);
        }
    }

    (0, 0)
}

fn dfs(
    grid: &[Vec<char>],
    walls: &HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
) -> usize {
    let mut stack = vec![(start, 0)];
    let mut visited = HashSet::new();

    while let Some((pos, time)) = stack.pop() {
        if pos == end {
            return time;
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let (r, c) = pos;
        if r > 0 && !walls.contains(&(r - 1, c)) {
            stack.push(((r - 1, c), time + 1));
        }
        if r < (grid.len() - 1) as i64 && !walls.contains(&(r + 1, c)) {
            stack.push(((r + 1, c), time + 1));
        }
        if c > 0 && !walls.contains(&(r, c - 1)) {
            stack.push(((r, c - 1), time + 1));
        }
        if c < (grid[r as usize].len() - 1) as i64 && !walls.contains(&(r, c + 1)) {
            stack.push(((r, c + 1), time + 1));
        }
    }

    0
}

fn bfs(
    grid: &[Vec<char>],
    walls: &HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
) -> usize {
    let mut queue = vec![(start, 0)];
    let mut visited = HashSet::new();

    while let Some((pos, time)) = queue.pop() {
        if pos == end {
            return time;
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        let (r, c) = pos;
        if r > 0 && !walls.contains(&(r - 1, c)) {
            queue.push(((r - 1, c), time + 1));
        }
        if r < (grid.len() - 1) as i64 && !walls.contains(&(r + 1, c)) {
            queue.push(((r + 1, c), time + 1));
        }
        if c > 0 && !walls.contains(&(r, c - 1)) {
            queue.push(((r, c - 1), time + 1));
        }
        if c < (grid[r as usize].len() - 1) as i64 && !walls.contains(&(r, c + 1)) {
            queue.push(((r, c + 1), time + 1));
        }
    }

    0
}

fn cheats_bfs(
    grid: &[Vec<char>],
    path: &HashSet<(i64, i64)>,
    walls: &HashSet<(i64, i64)>,
    start: (i64, i64),
    end: (i64, i64),
    limit: usize,
) -> usize {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut visited = HashSet::new();

    let mut dist: HashMap<(i64, i64), usize> = HashMap::new();
    let mut backlink: HashMap<(i64, i64), (i64, i64)> = HashMap::new();

    while let Some((pos, time)) = queue.pop_front() {
        if pos == end {
            break;
        }

        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        for dir in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            let next_time = time + 1;
            if !path.contains(&next) {
                continue;
            }

            if !visited.contains(&next) {
                let entry = dist.entry(pos).or_insert(usize::MAX);
                if next_time < *entry {
                    *entry = next_time;
                    backlink.insert(next, pos);
                }
            }
            queue.push_back((next, next_time + 1));
        }
    }

    let mut path = VecDeque::from([end]);
    let mut current = end;
    while let Some(entry) = backlink.get(&current) {
        path.push_front(*entry);
        current = *entry;
    }

    println!("{:?}", path.len());

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for i in 0..path.len() - 1 {
        for j in i + 1..path.len() {
            let dist = manhattan_distance(path[i], path[j]) as usize;
            if dist < 21 && (j - i) > dist {
                *cheats.entry((j - i) - dist).or_default() += 1;
            }
        }
    }

    let count: usize = cheats
        .iter()
        .filter_map(|(k, v)| if *k >= limit { Some(v) } else { None })
        .sum();

    count
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
