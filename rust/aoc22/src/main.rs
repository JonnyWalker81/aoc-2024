use std::collections::HashMap;
use std::fmt::{self, Display};
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
    let secrets: Vec<u64> = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    // println!("{:?}", secrets);

    let mut sum = 0;
    for secret in secrets {
        let mut next = secret;
        for _ in 0..2000 {
            next = next_secret(next);
        }
        // println!("{}", next);
        sum += next;
    }

    println!("{}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let secrets: Vec<u64> = input
        .trim()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    let all_buyer_prices = generate_buyer_prices(&secrets);

    // 3) For each buyer, generate the 2000 differences in [−9..=9].
    let all_buyer_diffs = generate_buyer_diffs(&all_buyer_prices);

    // 4) Invert the search: gather each 4-diff quadruple that actually appears
    //    (earliest occurrence per buyer), and sum up the selling prices for each quadruple.
    let map = accumulate_earliest_quadruples(&all_buyer_prices, &all_buyer_diffs);

    // 5) The answer is the largest total bananas among all quadruples that appear
    let best_sum = map.values().max().unwrap_or(&0);
    println!("{}", best_sum);

    Ok(())
}

fn next_secret(mut secret: u64) -> u64 {
    let next = secret * 64;
    secret ^= next;
    secret %= 16777216;

    let next2 = secret / 32;

    secret ^= next2;

    secret %= 16777216;

    let next3 = secret * 2048;

    secret ^= next3;

    secret %= 16777216;

    secret
}

/// Generate 2001 prices (ones digit) for each buyer:
/// - The initial secret's ones digit
/// - Followed by the ones digit of the next 2000 secrets
fn generate_buyer_prices(secrets: &[u64]) -> Vec<Vec<u8>> {
    let mut all_buyer_prices = Vec::with_capacity(secrets.len());

    for &initial_secret in secrets {
        let mut secret = initial_secret;
        // We'll store the ones digit of each secret
        let mut prices = Vec::with_capacity(2001);

        // Price #0: from the initial secret
        prices.push((secret % 10) as u8);

        // Generate 2000 more secrets
        for _ in 0..2000 {
            secret = next_secret(secret);
            prices.push((secret % 10) as u8);
        }
        all_buyer_prices.push(prices);
    }

    all_buyer_prices
}

/// Generate 2000 differences for each buyer (since each buyer has 2001 prices).
/// differences[i] = prices[i+1] - prices[i] in the range [−9..=9].
fn generate_buyer_diffs(all_buyer_prices: &[Vec<u8>]) -> Vec<Vec<i8>> {
    let mut all_buyer_diffs = Vec::with_capacity(all_buyer_prices.len());

    for prices in all_buyer_prices {
        let mut diffs = Vec::with_capacity(prices.len() - 1);
        for i in 0..(prices.len() - 1) {
            let diff = prices[i + 1] as i8 - prices[i] as i8;
            diffs.push(diff);
        }
        all_buyer_diffs.push(diffs);
    }

    all_buyer_diffs
}

/// For each buyer, find *all* quadruples of consecutive differences, record the
/// *earliest index* at which they appear (because the buyer sells immediately),
/// and accumulate the buyer’s selling price (ones digit) in a global map.
/// Map key: [d0, d1, d2, d3], Map value: total bananas across all buyers.
fn accumulate_earliest_quadruples(
    all_buyer_prices: &[Vec<u8>],
    all_buyer_diffs: &[Vec<i8>],
) -> HashMap<[i8; 4], u64> {
    let mut map = HashMap::new();

    // Loop over each buyer
    for (prices, diffs) in all_buyer_prices.iter().zip(all_buyer_diffs) {
        // We'll track the earliest occurrence of each quadruple for this buyer
        // so we only add that buyer's sell price once per quadruple.
        let mut earliest_quad = HashMap::<[i8; 4], usize>::new();

        // Each buyer has 2000 diffs, so we can form 1997 quadruples (0..=1996)
        if diffs.len() >= 4 {
            for i in 0..=(diffs.len() - 4) {
                let quad = [diffs[i], diffs[i + 1], diffs[i + 2], diffs[i + 3]];
                earliest_quad.entry(quad).or_insert(i);
            }
        }

        // For each quadruple that occurred in this buyer's stream,
        // add the buyer's selling price at index `i+4`
        for (&quad, &start_idx) in &earliest_quad {
            // The actual selling price is at prices[start_idx + 4]
            let sell_price = prices[start_idx + 4] as u64;
            *map.entry(quad).or_insert(0) += sell_price;
        }
    }

    map
}
