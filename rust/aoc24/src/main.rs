use std::collections::HashMap;
use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let (top, bottom) = input.trim().split_once("\n\n").unwrap();

    let mut circuit = HashMap::new();
    let mut wire_gates = HashMap::new();
    let wires: Vec<Wire> = top
        .trim()
        .lines()
        .map(|l| {
            let (name, value) = l.split_once(": ").unwrap();
            let w = Wire {
                name: name.to_string(),
                value: value.parse().unwrap(),
            };
            circuit.insert(w.name.clone(), w.clone());
            w
        })
        .collect();

    let gates: Vec<Gate> = bottom
        .trim()
        .lines()
        .map(|l| {
            let (input, output) = l.split_once(" -> ").unwrap();
            let inputs = input.split(" ").collect::<Vec<_>>();
            let g = Gate {
                input1: inputs[0].to_string(),
                op: inputs[1].into(),
                input2: inputs[2].to_string(),
                output: output.to_string(),
            };
            wire_gates.insert(g.output.clone(), g.clone());
            g
        })
        .collect();

    let mut z_wires = wire_gates
        .keys()
        .filter(|w| w.starts_with("z"))
        .collect::<Vec<_>>();
    z_wires.sort();

    println!("wires: {:?}", z_wires);
    // let mut outputs = HashMap::new();
    let mut bit_str = String::new();
    for wire in z_wires {
        let bit = get_value(wire.as_str(), &mut circuit, &wire_gates);
        bit_str = format!("{}{}", bit, bit_str);
    }

    println!("bit_str: {}", bit_str);
    let val = usize::from_str_radix(bit_str.as_str(), 2).unwrap();
    println!("val: {}", val);

    Ok(())
}

fn get_value(
    name: &str,
    circuit: &mut HashMap<String, Wire>,
    wire_gates: &HashMap<String, Gate>,
) -> u16 {
    if circuit.contains_key(name) {
        return circuit.get(name).unwrap().value;
    }

    let gate = wire_gates.get(name).unwrap();

    let val1 = get_value(gate.input1.as_str(), circuit, wire_gates);
    let val2 = get_value(gate.input2.as_str(), circuit, wire_gates);

    let val = match gate.op {
        Op::And => val1 & val2,
        Op::Or => val1 | val2,
        Op::Xor => val1 ^ val2,
    };

    circuit.insert(
        name.to_string(),
        Wire {
            name: name.to_string(),
            value: val,
        },
    );

    val
}

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: u16,
}

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    output: String,
    op: Op,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => panic!("Unknown op: {}", s),
        }
    }
}
