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
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // println!("{:?}", grid);

    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let xmas = ['X', 'M', 'A', 'S'];
    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            for &(dx, dy) in &dirs {
                let mut x = i as isize;
                let mut y = j as isize;
                let mut k = 0;

                while k < xmas.len() {
                    if x < 0 || x >= grid.len() as isize || y < 0 || y >= grid[0].len() as isize {
                        break;
                    }

                    if grid[x as usize][y as usize] != xmas[k] {
                        break;
                    }

                    x += dx;
                    y += dy;
                    k += 1;
                }

                if k == xmas.len() {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // println!("{:?}", grid);

    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 'A' {
                // check the for corners for 'S' or 'M'
                // Positions for the diagonals
                let positions = [
                    ((i as isize) - 1, (j as isize) - 1), // upper-left
                    ((i as isize) - 1, (j as isize) + 1), // upper-right
                    ((i as isize) + 1, (j as isize) - 1), // lower-left
                    ((i as isize) + 1, (j as isize) + 1), // lower-right
                ];

                // Check if positions are within bounds
                if positions.iter().any(|&(x, y)| {
                    x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize
                }) {
                    continue;
                }

                // Extract the characters at the diagonal positions
                let ul = grid[(i - 1)][(j - 1)];
                let ur = grid[(i - 1)][(j + 1)];
                let ll = grid[(i + 1)][(j - 1)];
                let lr = grid[(i + 1)][(j + 1)];

                // Check first diagonal (upper-left to lower-right)
                let arm1_valid = ((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M'));

                // Check second diagonal (upper-right to lower-left)
                let arm2_valid = ((ur == 'M' && ll == 'S') || (ur == 'S' && ll == 'M'));

                // If both arms form MAS or SAM sequences, increment count
                if arm1_valid && arm2_valid {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    Ok(())
}
