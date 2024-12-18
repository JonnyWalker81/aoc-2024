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

#[derive(Debug, Clone, PartialEq)]
struct Group {
    char: char,
    area: HashSet<(usize, usize)>,
}

fn part1(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.chars().collect()).collect();

    // group aread of same char
    let groups = group_areas(&grid);

    // println!("{:?}", groups.len());
    // println!("{:?}", groups);

    let mut perimete_pairs = vec![];
    // for each group count the permieter
    for group in groups.iter() {
        let mut perimeter = 0;

        for (r, c) in group.area.iter() {
            let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

            for (dx, dy) in dirs.iter() {
                let nx = *r as i32 + dx;
                let ny = *c as i32 + dy;

                if nx < 0 || ny < 0 {
                    perimeter += 1;
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if nx >= grid.len() || ny >= grid[0].len() || !group.area.contains(&(nx, ny)) {
                    perimeter += 1;
                }
            }
        }

        // println!("{:?} {:?}", group.char, perimeter);
        perimete_pairs.push((group.area.len(), perimeter));
    }

    let mut perimeter = 0;
    for (area, p) in perimete_pairs.iter() {
        perimeter += (area * p);
    }

    println!("{:?}", perimeter);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut p1 = 0;
    let mut p2 = 0;

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for r in 0..rows {
        for c in 0..cols {
            if seen.contains(&(r, c)) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back((r, c));

            let mut area = 0;
            let mut perimeter = 0;
            let mut perim_map: HashMap<(isize, isize), HashSet<(usize, usize)>> = HashMap::new();

            while let Some((r2, c2)) = queue.pop_front() {
                if seen.contains(&(r2, c2)) {
                    continue;
                }

                seen.insert((r2, c2));
                area += 1;

                for &(dr, dc) in &dirs {
                    let rr = r2 as isize + dr;
                    let cc = c2 as isize + dc;

                    if rr >= 0 && rr < rows as isize && cc >= 0 && cc < cols as isize {
                        let rr = rr as usize;
                        let cc = cc as usize;

                        if grid[rr][cc] == grid[r2][c2] {
                            queue.push_back((rr, cc));
                        } else {
                            perimeter += 1;
                            perim_map.entry((dr, dc)).or_default().insert((r2, c2));
                        }
                    } else {
                        perimeter += 1;
                        perim_map.entry((dr, dc)).or_default().insert((r2, c2));
                    }
                }
            }

            let mut sides = 0;

            for (_dir, cells) in perim_map {
                let mut seen_perim: HashSet<(usize, usize)> = HashSet::new();

                for &(pr, pc) in &cells {
                    if seen_perim.contains(&(pr, pc)) {
                        continue;
                    }

                    sides += 1;
                    let mut queue = VecDeque::new();
                    queue.push_back((pr, pc));

                    while let Some((r2, c2)) = queue.pop_front() {
                        if seen_perim.contains(&(r2, c2)) {
                            continue;
                        }

                        seen_perim.insert((r2, c2));

                        for &(dr, dc) in &dirs {
                            let rr = r2 as isize + dr;
                            let cc = c2 as isize + dc;

                            if rr >= 0 && rr < rows as isize && cc >= 0 && cc < cols as isize {
                                let rr = rr as usize;
                                let cc = cc as usize;

                                if cells.contains(&(rr, cc)) {
                                    queue.push_back((rr, cc));
                                }
                            }
                        }
                    }
                }
            }

            p1 += area * perimeter;
            p2 += area * sides;
        }
    }

    println!("{:?}", p1);
    println!("{:?}", p2);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct State {
    char: char,
    current: (usize, usize),
}

fn group_areas(grid: &[Vec<char>]) -> Vec<Group> {
    let mut visited = HashSet::new();

    let mut groups = vec![];

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if visited.contains(&(r, c)) {
                continue;
            }

            let state = State {
                char: grid[r][c],
                current: (r, c),
            };
            let mut stack = vec![state];

            let mut area = HashSet::new();
            area.insert((r, c));

            while let Some(state) = stack.pop() {
                let (x, y) = state.current;

                // if visited.contains(&(x, y)) {
                //     continue;
                // }

                visited.insert((x, y));

                for (dx, dy) in dirs.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx < 0 || ny < 0 {
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    if nx >= grid.len() || ny >= grid[0].len() || visited.contains(&(nx, ny)) {
                        continue;
                    }

                    if grid[nx][ny] == state.char {
                        area.insert((nx, ny));
                        stack.push(State {
                            char: state.char,
                            current: (nx, ny),
                        });
                    }
                }
            }

            groups.push(Group {
                char: grid[r][c],
                area,
            });
        }
    }

    groups
}
