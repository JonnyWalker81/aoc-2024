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

fn min_taps_2(p: (i64, i64), game: &Game, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    if p == (0, 0) {
        return 0;
    }

    if p.0 < 0 || p.1 < 0 {
        return i64::MAX;
    }

    if p.0 > game.prize.0 || p.1 > game.prize.1 {
        return i64::MAX;
    }

    if let Some(&result) = memo.get(&(p.0, p.1)) {
        return result;
    }

    let mut cost_options = vec![];

    let prevX_A = p.0 - game.button_a.0;
    let prevY_A = p.1 - game.button_a.1;

    // println!("A: {} {} {} {}", p.0, p.1, prevX_A, prevY_A);
    if prevX_A >= 0 && prevY_A >= 0 {
        let cost_from_A = min_taps_2((prevX_A, prevY_A), game, memo);
        if cost_from_A != i64::MAX {
            cost_options.push(cost_from_A + COST_A)
        }
    }

    let prevX_B = p.0 - game.button_b.0;
    let prevY_B = p.1 - game.button_b.1;

    // println!("B: {} {} {} {}", p.0, p.1, prevX_B, prevY_B);
    if prevX_B >= 0 && prevY_B >= 0 {
        let cost_from_B = min_taps_2((prevX_B, prevY_B), game, memo);
        if cost_from_B != i64::MAX {
            cost_options.push(cost_from_B + COST_B)
        }
    }

    if cost_options.is_empty() {
        memo.insert((p.0, p.1), i64::MAX);
        return i64::MAX;
    }

    // println!("{:?}", cost_options);
    let best_cost = cost_options.iter().min().unwrap_or(&0);
    memo.insert((p.0, p.1), *best_cost);
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
                prize: (prize_x + 10_000_000_000_000, prize_y + 10_000_000_000_000),
            }
        })
        .collect::<Vec<Game>>();

    // println!("{:?}", games);

    // let x_a = 26;
    // let y_a = 66;
    // let x_b = 67;
    // let y_b = 21;
    // let x_prize = 10000000012748;
    // let y_prize = 10000000012176;

    // match solve_machine(x_a, y_a, x_b, y_b, x_prize, y_prize) {
    //     Some(cost) => println!("Minimum cost to win: {}", cost),
    //     None => println!("No solution to win the prize."),
    // }

    // let mut sum = 0;
    // // for game in games.iter() {
    // //     let result = solve_machine(
    // //         game.button_a.0,
    // //         game.button_a.1,
    // //         game.button_b.0,
    // //         game.button_b.1,
    // //         game.prize.0,
    // //         game.prize.1,
    // //     );

    // //     if let Some(cost) = result {
    // //         sum += cost as i64;
    // //     }
    // // }

    // println!("Sum: {}", sum);
    Ok(())
}

fn solve_machine(
    x_a: i64,
    y_a: i64,
    x_b: i64,
    y_b: i64,
    x_prize: i64,
    y_prize: i64,
) -> Option<i64> {
    // Compute the determinant
    let det = x_a * y_b - y_a * x_b;

    if det == 0 {
        // Handle singular case
        println!("Singular system (det = 0). No unique solution.");
        return None;
    }

    // Compute the numerators for a_0 and b_0
    let a0_num = x_prize * y_b - y_prize * x_b;
    let b0_num = x_a * y_prize - y_a * x_prize;

    // Check if the numerators are divisible by the determinant
    if a0_num % det != 0 || b0_num % det != 0 {
        println!("No integer solution exists.");
        return None;
    }

    // Compute the integer solutions a_0 and b_0
    let a0 = a0_num / det;
    let b0 = b0_num / det;

    // Ensure gcd of x_a and x_b
    let g = x_a.gcd(&x_b);

    // Compute the general solution parameters
    let u = x_b / g;
    let v = x_a / g;

    // Enforce nonnegativity of a(t) and b(t)
    let t_min_a = if u > 0 {
        (-a0).div_ceil(&u)
    } else {
        (-a0).div_floor(&u)
    };
    let t_max_b = if v > 0 {
        (b0).div_floor(&v)
    } else {
        (b0).div_ceil(&v)
    };

    // Determine the range of valid t
    let t_min = t_min_a.max(t_max_b);
    let t_max = t_min_a.min(t_max_b);

    if t_min > t_max {
        println!("No valid nonnegative solutions for a(t) and b(t).");
        return None;
    }

    // Cost function: cost(t) = 3a(t) + b(t)
    let k = 3 * u - v;
    let t_best = if k > 0 {
        t_min
    } else if k < 0 {
        t_max
    } else {
        t_min // If k == 0, cost is constant for all valid t
    };

    // Compute the final a and b values for t_best
    let a_best = a0 + t_best * u;
    let b_best = b0 - t_best * v;
    let cost_best = 3 * a_best + b_best;

    Some(cost_best)
}
