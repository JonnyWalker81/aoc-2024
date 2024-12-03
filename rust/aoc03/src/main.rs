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
    let mut pos = 0;
    let mut memory = vec![];
    'outer: while pos < input.len() {
        let c = input.chars().nth(pos).unwrap();
        match c {
            'm' => {
                if input.chars().nth(pos + 1) == Some('u')
                    && input.chars().nth(pos + 2) == Some('l')
                    && input.chars().nth(pos + 3) == Some('(')
                {
                    let mut p = pos + 4;
                    let mut inner = String::new();
                    while let Some(cc) = input.chars().nth(p) {
                        if p == input.len() {
                            break;
                        }

                        if cc.is_ascii_digit() || cc == ',' {
                            inner.push(cc);
                        } else {
                            if cc == ')' {
                                break;
                            } else {
                                pos = p;
                                continue 'outer;
                            }
                        }
                        p += 1;
                    }
                    pos = p;
                    // println!("Found the word 'mul' followed by '(' -> {}", inner);
                    memory.push(inner);
                }
            }
            _ => (),
        }
        pos += 1;
    }

    let mut sum = 0;
    for m in memory {
        let mut nums = m.split(',');
        let a = nums.next().unwrap().parse::<i32>().unwrap();
        let b = nums.next().unwrap().parse::<i32>().unwrap();
        sum += a * b;
    }

    println!("Sum: {}", sum);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut pos = 0;
    let mut memory = vec![];
    let mut is_dont = false;
    let mut is_do = false;
    'outer: while pos < input.len() {
        let c = input.chars().nth(pos).unwrap();
        match c {
            'm' => {
                if input.chars().nth(pos + 1) == Some('u')
                    && input.chars().nth(pos + 2) == Some('l')
                    && input.chars().nth(pos + 3) == Some('(')
                {
                    let mut p = pos + 4;
                    let mut inner = String::new();
                    while let Some(cc) = input.chars().nth(p) {
                        if p == input.len() {
                            break;
                        }

                        if cc.is_ascii_digit() || cc == ',' {
                            inner.push(cc);
                        } else {
                            if cc == ')' {
                                break;
                            } else {
                                pos = p;
                                continue 'outer;
                            }
                        }
                        p += 1;
                    }
                    pos = p;
                    // println!("Found the word 'mul' followed by '(' -> {}", inner);
                    if !is_dont || is_do {
                        memory.push(inner);
                    }
                }
            }
            'd' => {
                is_dont = false;
                is_do = false;
                if input.chars().nth(pos + 1) == Some('o') {
                    if input.chars().nth(pos + 2) == Some('(') {
                        is_do = true;
                        pos += 3;
                    } else if input.chars().nth(pos + 2) == Some('n')
                        && input.chars().nth(pos + 3) == Some('\'')
                        && input.chars().nth(pos + 4) == Some('t')
                    {
                        is_dont = true;
                        pos += 5;
                    }
                }
            }
            _ => (),
        }
        pos += 1;
    }

    let mut sum = 0;
    for m in memory {
        let mut nums = m.split(',');
        let a = nums.next().unwrap().parse::<i32>().unwrap();
        let b = nums.next().unwrap().parse::<i32>().unwrap();
        sum += a * b;
    }

    println!("Sum: {}", sum);
    Ok(())
}
