use nalgebra::{Matrix2, Vector2};
use num_integer::Integer;
use std::collections::HashMap;
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
struct Game {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl Game {
    fn calc_tokens_with_scale(&self, scale: i64) -> Option<i64> {
        let a = self.button_a;
        let b = self.button_b;
        let prize = (self.prize.0 + scale, self.prize.1 + scale);

        let n = (a.0 * prize.1 - a.1 * prize.0) / (a.0 * b.1 - a.1 * b.0);
        let m = (prize.0 - b.0 * n) / a.0;

        if (a.0 * m + b.0 * n, a.1 * m + b.1 * n) == prize {
            Some(3 * m + n)
        } else {
            None
        }
    }

    fn calc_tokens(&self) -> Option<i64> {
        self.calc_tokens_with_scale(0)
    }

    fn calc_scaled(&self) -> Option<i64> {
        self.calc_tokens_with_scale(10_000_000_000_000)
    }
}

// Sample input:
// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400
fn part1(input: &str) -> Result<()> {
    let chunked: Vec<&str> = input.trim().split("\n\n").collect();
    let games = chunked
        .iter()
        .map(|chunk| {
            let mut lines = chunk.lines();
            let button_a = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
            let button_b = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
            let prize = lines.next().unwrap().split(", ").collect::<Vec<&str>>();

            let button_a_x = button_a[0].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_a_y = button_a[1].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_b_x = button_b[0].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_b_y = button_b[1].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let prize_x = prize[0].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let prize_y = prize[1].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();

            Game {
                button_a: (button_a_x, button_a_y),
                button_b: (button_b_x, button_b_y),
                prize: (prize_x, prize_y),
            }
        })
        .collect::<Vec<Game>>();

    // println!("{:?}", games);

    let mut memo = HashMap::new();
    let mut sum = 0;
    for game in games.iter() {
        let min_taps = min_taps(game.prize, &game, &mut memo, 0, 0, 100);

        if min_taps == i64::MAX {
            // println!("No solution found");
        } else {
            // println!("Min taps: {}", min_taps);
            sum += min_taps;
        }
    }

    println!("Sum: {}", sum);
    Ok(())
}

const COST_A: i64 = 3;
const COST_B: i64 = 1;
const PRESS_LIMIT: i64 = 100;

fn min_taps(
    p: (i64, i64),
    game: &Game,
    memo: &mut HashMap<(i64, i64, i64, i64), i64>,
    a_count: i64,
    b_count: i64,
    limit: i64,
) -> i64 {
    if p == (0, 0) {
        return 0;
    }

    if a_count > limit || b_count > limit {
        return i64::MAX;
    }

    if p.0 < 0 || p.1 < 0 {
        return i64::MAX;
    }

    if p.0 > game.prize.0 || p.1 > game.prize.1 {
        return i64::MAX;
    }

    if let Some(&result) = memo.get(&(p.0, p.1, a_count, b_count)) {
        return result;
    }

    let mut cost_options = vec![];

    let prevX_A = p.0 - game.button_a.0;
    let prevY_A = p.1 - game.button_a.1;

    // println!("A: {} {} {} {}", p.0, p.1, prevX_A, prevY_A);
    if prevX_A >= 0 && prevY_A >= 0 {
        let cost_from_A = min_taps((prevX_A, prevY_A), game, memo, a_count + 1, b_count, limit);
        if cost_from_A != i64::MAX {
            cost_options.push(cost_from_A + COST_A)
        }
    }

    let prevX_B = p.0 - game.button_b.0;
    let prevY_B = p.1 - game.button_b.1;

    // println!("B: {} {} {} {}", p.0, p.1, prevX_B, prevY_B);
    if prevX_B >= 0 && prevY_B >= 0 {
        let cost_from_B = min_taps((prevX_B, prevY_B), game, memo, a_count, b_count + 1, limit);
        if cost_from_B != i64::MAX {
            cost_options.push(cost_from_B + COST_B)
        }
    }

    if cost_options.is_empty() {
        memo.insert((p.0, p.1, a_count, b_count), i64::MAX);
        return i64::MAX;
    }

    // println!("{:?}", cost_options);
    let best_cost = cost_options.iter().min().unwrap_or(&0);
    memo.insert((p.0, p.1, a_count, b_count), *best_cost);
    *best_cost
}

fn part2(input: &str) -> Result<()> {
    let chunked: Vec<&str> = input.trim().split("\n\n").collect();
    let games = chunked
        .iter()
        .map(|chunk| {
            let mut lines = chunk.lines();
            let button_a = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
            let button_b = lines.next().unwrap().split(", ").collect::<Vec<&str>>();
            let prize = lines.next().unwrap().split(", ").collect::<Vec<&str>>();

            let button_a_x = button_a[0].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_a_y = button_a[1].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_b_x = button_b[0].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let button_b_y = button_b[1].split("+").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let prize_x = prize[0].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();
            let prize_y = prize[1].split("=").collect::<Vec<&str>>()[1]
                .parse::<i64>()
                .unwrap();

            Game {
                button_a: (button_a_x, button_a_y),
                button_b: (button_b_x, button_b_y),
                prize: (prize_x, prize_y),
            }
        })
        .collect::<Vec<Game>>();

    let mut sum = 0;
    for game in games.iter() {
        if let Some(min_taps) = game.calc_scaled() {
            sum += min_taps;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}
