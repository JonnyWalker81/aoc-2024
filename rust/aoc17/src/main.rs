use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    // let mut input = String::new();
    // io::stdin().read_to_string(&mut input)?;

    // part1("0,1,5,4,3,0")?;
    part1("2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0")?;
    part2("2,4,1,6,7,5,4,6,1,4,5,5,0,3,3,0")?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut registers = HashMap::new();
    // registers.insert("A", 729);
    registers.insert("A", 66171486);
    registers.insert("B", 0);
    registers.insert("C", 0);

    let program: Vec<usize> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let out = execute(&program, &mut registers);
    let output: String = out.iter().map(|x| x.to_string()).collect();
    println!("{}", output);

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut registers = HashMap::new();
    // registers.insert("A", 729);
    registers.insert("A", 2024);
    registers.insert("B", 0);
    registers.insert("C", 0);

    let program: Vec<usize> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut saved = vec![];
    for i in 1..1024 {
        registers.insert("A", i);
        registers.insert("B", 0);
        registers.insert("C", 0);
        let output = execute(&program, &mut registers);

        if output[0] == program[0] {
            saved.push(i);
        }
    }

    let mut pos = 1;
    while pos < program.len() {
        let mut next = vec![];
        for v in saved {
            for bit in 0..8 {
                let num = (bit << (7 + 3 * pos)) | v;
                registers.insert("A", num);
                registers.insert("B", 0);
                registers.insert("C", 0);

                let output = execute(&program, &mut registers);

                if output.len() > pos && output[pos] == program[pos] {
                    next.push(num);
                }
            }
        }
        saved = next;
        pos += 1;
    }

    let min = saved.iter().min().unwrap();
    println!("{}", *min);

    Ok(())
}

fn execute(program: &[usize], registers: &mut HashMap<&str, usize>) -> Vec<usize> {
    let mut out = vec![];

    let mut pc = 0;
    loop {
        if pc >= program.len() {
            break;
        }

        let instr = program[pc];
        let mut operand = program[pc + 1];
        match instr {
            0 => {
                // adv
                let numerator = registers.get("A").unwrap();
                operand = operand_to_value(operand, &registers);
                let denominator = 2_usize.pow(operand as u32);
                let quotient = numerator / denominator;
                registers.insert("A", quotient);
            }
            1 => {
                // bxl
                let value = registers.get("B").unwrap();
                let bitwise = value ^ operand;
                registers.insert("B", bitwise);
            }
            2 => {
                // bst
                operand = operand_to_value(operand, &registers);
                let value = operand % 8;
                registers.insert("B", value);
            }
            3 => {
                // jnz
                let value = registers.get("A").unwrap();
                if *value != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => {
                // bxc
                let left = registers.get("B").unwrap();
                let right = registers.get("C").unwrap();
                let bitwise = left ^ right;
                registers.insert("B", bitwise);
            }
            5 => {
                // out
                operand = operand_to_value(operand, &registers);
                let value = operand % 8;
                out.push(value);
            }
            6 => {
                // bdv
                operand = operand_to_value(operand, &registers);
                let numerator = registers.get("A").unwrap();
                let denominator = 2_usize.pow(operand as u32);
                let quotient = numerator / denominator;
                registers.insert("B", quotient);
            }
            7 => {
                // cdv
                operand = operand_to_value(operand, &registers);
                let numerator = registers.get("A").unwrap();
                let denominator = 2_usize.pow(operand as u32);
                let quotient = numerator / denominator;
                registers.insert("C", quotient);
            }
            _ => {
                println!("Unknown instruction: {}", instr);
                break;
            }
        }

        pc += 2;
    }

    out
}

fn operand_to_value(operand: usize, registers: &HashMap<&str, usize>) -> usize {
    match operand {
        0..=3 => operand,
        4 => *registers.get("A").unwrap(),
        5 => *registers.get("B").unwrap(),
        6 => *registers.get("C").unwrap(),
        7 => operand,
        _ => panic!("Unknown operand: {}", operand),
    }
}
