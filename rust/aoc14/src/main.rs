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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

// p=position v=velocity
// p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3
fn part1(input: &str) -> Result<()> {
    let mut robots: Vec<Robot> = input
        .trim()
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let p = p.split_once("p=").unwrap().1;
            let v = v.split_once("v=").unwrap().1;
            let p = p.split_once(",").unwrap();
            let v = v.split_once(",").unwrap();
            Robot {
                position: (p.0.parse().unwrap(), p.1.parse().unwrap()),
                velocity: (v.0.parse().unwrap(), v.1.parse().unwrap()),
            }
        })
        .collect();

    let width = 101;
    let height = 103;

    for _ in 0..100 {
        for r in 0..robots.len() {
            let robot = &mut robots[r];
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(width);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(height);
        }
    }

    let x_midpoint = width / 2;
    let y_midpoint = height / 2;
    let upper_left_quadrant_x = (0, x_midpoint);
    let upper_left_quadrant_y = (0, y_midpoint);

    let upper_right_quadrant_x = (x_midpoint, width);
    let upper_right_quadrant_y = (0, y_midpoint);

    let lower_left_quadrant_x = (0, x_midpoint);
    let lower_left_quadrant_y = (y_midpoint, height);

    let lower_right_quadrant_x = (x_midpoint, width);
    let lower_right_quadrant_y = (y_midpoint, height);

    let mut quadrant_counts = [0; 4];

    println!("midpoint: ({}, {})", x_midpoint, y_midpoint);
    for r in &robots {
        println!("{:?}", r.position);
        if r.position.0 == x_midpoint || r.position.1 == y_midpoint {
            println!("midpoint");
            continue;
        }

        // upper left quadrant
        if upper_left_quadrant_x.0 <= r.position.0
            && r.position.0 < upper_left_quadrant_x.1
            && upper_left_quadrant_y.0 <= r.position.1
            && r.position.1 < upper_left_quadrant_y.1
        {
            quadrant_counts[0] += 1;
        } else if upper_right_quadrant_x.0 <= r.position.0
            && r.position.0 < upper_right_quadrant_x.1
            && upper_right_quadrant_y.0 <= r.position.1
            && r.position.1 < upper_right_quadrant_y.1
        {
            quadrant_counts[1] += 1;
        } else if lower_left_quadrant_x.0 <= r.position.0
            && r.position.0 < lower_left_quadrant_x.1
            && lower_left_quadrant_y.0 <= r.position.1
            && r.position.1 < lower_left_quadrant_y.1
        {
            quadrant_counts[2] += 1;
        } else if lower_right_quadrant_x.0 <= r.position.0
            && r.position.0 < lower_right_quadrant_x.1
            && lower_right_quadrant_y.0 <= r.position.1
            && r.position.1 < lower_right_quadrant_y.1
        {
            quadrant_counts[3] += 1;
        }
    }

    println!("{:?}", quadrant_counts);

    let sum = quadrant_counts.iter().product::<i32>();
    println!("{}", sum);

    Ok(())
}

fn print_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let grid: HashSet<(i32, i32)> = robots.iter().map(|r| r.position).collect();

    for r in 0..width {
        for c in 0..height {
            if grid.contains(&(r, c)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part2(input: &str) -> Result<()> {
    let mut robots: Vec<Robot> = input
        .trim()
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" ").unwrap();
            let p = p.split_once("p=").unwrap().1;
            let v = v.split_once("v=").unwrap().1;
            let p = p.split_once(",").unwrap();
            let v = v.split_once(",").unwrap();
            Robot {
                position: (p.0.parse().unwrap(), p.1.parse().unwrap()),
                velocity: (v.0.parse().unwrap(), v.1.parse().unwrap()),
            }
        })
        .collect();

    let width = 101;
    let height = 103;

    let mut v = 103;
    let mut h = 175;
    for s in 0..10000 {
        for r in 0..robots.len() {
            let robot = &mut robots[r];
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(width);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(height);
        }

        if s % height == 0 || s % width == 0 {
            println!("after: {}", s + 1);
            print_robots(&robots, width, height);
        }
    }

    Ok(())
}
