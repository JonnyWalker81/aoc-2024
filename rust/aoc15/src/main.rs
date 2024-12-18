use std::collections::HashSet;
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
    let (g, m) = input.trim().split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<char>> = g
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let moves: Vec<char> = m.trim().chars().filter(|c| !c.is_whitespace()).collect();

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();
    let walls: HashSet<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '@' {
                pos = (r, c);
                grid[r][c] = '.';
            } else if grid[r][c] == 'O' {
                boxes.insert((r, c));
            }
        }
    }

    for m in &moves {
        match m {
            '>' => move_box(&grid, &mut pos, &walls, &mut boxes, (0, 1)),
            '<' => move_box(&grid, &mut pos, &walls, &mut boxes, (0, -1)),
            '^' => move_box(&grid, &mut pos, &walls, &mut boxes, (-1, 0)),
            'v' => move_box(&grid, &mut pos, &walls, &mut boxes, (1, 0)),
            _ => panic!("invalid move"),
        }
    }

    // print_grid(&grid, pos, &boxes, &walls);

    let mut sum = 0;
    for b in boxes {
        sum += 100 * b.0 + b.1;
    }

    println!("Sum: {}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let (g, m) = input.trim().split_once("\n\n").unwrap();
    let mut grid: Vec<Vec<char>> = g
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    let moves: Vec<char> = m.trim().chars().filter(|c| !c.is_whitespace()).collect();

    // expand grid
    for r in 0..grid.len() {
        let mut c = 0;
        loop {
            if c >= grid[r].len() * 2 {
                break;
            }

            if c < grid[r].len() && grid[r][c] == 'O' {
                println!("O at ({}, {})", r, c);
            }

            if grid[r][c] == '#' {
                // insert '#' to the right
                grid[r].insert(c + 1, '#');
            } else if grid[r][c] == 'O' {
                println!("O at ({}, {})", r, c);
                // replace 'O' with '[]'
                grid[r][c] = '[';
                grid[r].insert(c + 1, ']');
            } else if grid[r][c] == '.' {
                grid[r].insert(c + 1, '.');
            }
            c += 2;
        }
    }

    println!("width: {}", grid[0].len());
    println!("height: {}", grid.len());

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            print!("{}", grid[r][c]);
        }
        println!();
    }

    let mut pos = (0, 0);
    let mut boxes = HashSet::new();
    let walls: HashSet<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c == '#')
                .map(move |(c, _)| (r, c))
        })
        .collect();

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '@' {
                pos = (r, c);
                grid[r][c] = '.';
            } else if grid[r][c] == 'O' {
                boxes.insert((r, c));
            }
        }
    }

    // for m in &moves {
    //     match m {
    //         '>' => move_box(&grid, &mut pos, &walls, &mut boxes, (0, 1)),
    //         '<' => move_box(&grid, &mut pos, &walls, &mut boxes, (0, -1)),
    //         '^' => move_box(&grid, &mut pos, &walls, &mut boxes, (-1, 0)),
    //         'v' => move_box(&grid, &mut pos, &walls, &mut boxes, (1, 0)),
    //         _ => panic!("invalid move"),
    //     }
    // }

    print_grid(&grid, pos, &boxes, &walls);

    // let mut sum = 0;
    // for b in boxes {
    //     sum += 100 * b.0 + b.1;
    // }

    // println!("Sum: {}", sum);
    Ok(())
}

fn is_empty_cell(
    pos: (usize, usize),
    walls: &HashSet<(usize, usize)>,
    boxes: &HashSet<(usize, usize)>,
) -> bool {
    !walls.contains(&pos) && !boxes.contains(&pos)
}

fn print_grid(
    grid: &Vec<Vec<char>>,
    pos: (usize, usize),
    boxes: &HashSet<(usize, usize)>,
    walls: &HashSet<(usize, usize)>,
) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if pos == (r, c) {
                print!("@");
            } else if boxes.contains(&(r, c)) {
                print!("O");
            } else if walls.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn move_box(
    grid: &[Vec<char>],
    pos: &mut (usize, usize),
    walls: &HashSet<(usize, usize)>,
    boxes: &mut HashSet<(usize, usize)>,
    dir: (isize, isize),
) {
    let next_pos = (
        (pos.0 as isize + dir.0) as usize,
        (pos.1 as isize + dir.1) as usize,
    );

    if next_pos.0 < grid.len()
        && next_pos.1 < grid[next_pos.0].len()
        && is_empty_cell(next_pos, walls, boxes)
    {
        // Move player to empty cell
        pos.0 = next_pos.0;
        pos.1 = next_pos.1;
    } else if next_pos.0 < grid.len()
        && next_pos.1 < grid[next_pos.0].len()
        && boxes.contains(&next_pos)
    {
        // Handle box movement
        let mut next_box_pos = next_pos;
        loop {
            next_box_pos = (
                (next_box_pos.0 as isize + dir.0) as usize,
                (next_box_pos.1 as isize + dir.1) as usize,
            );

            if next_box_pos.0 >= grid.len()
                || next_box_pos.1 >= grid[next_box_pos.0].len()
                || walls.contains(&next_box_pos)
            {
                break;
            }

            if !boxes.contains(&next_box_pos) {
                // Found an empty spot for the box
                boxes.remove(&next_pos);
                boxes.insert(next_box_pos);
                pos.0 = next_pos.0;
                pos.1 = next_pos.1;
                break;
            }
        }
    }
}
