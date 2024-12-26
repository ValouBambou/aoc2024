use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> String {
    let (registers, binary) = parse(input);
    let mut output = Vec::with_capacity(binary.len());
    run_program(&binary, registers, &mut output, false);
    output
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

/// Here we must reverse the program
/// so the example is :
/// ```python
/// while A > 0:
///     A /= 8
///     print(A % 8)
/// ```
/// The real program is a bit more complicated
/// ```python
/// while A > 0:
///    B = A % 8
///    B ^= 7
///    C = A / (2 ** B)
///    A /= 8
///    B ^= C
///    B ^= 7
///    print(B % 8)
/// ```
/// So only starting last 3 bits are used at each iteration
fn part2(input: &'static str) -> u64 {
    let (mut registers, binary) = parse(input);
    registers[A] = 0;
    let mut output = Vec::with_capacity(binary.len());
    let mut cache = HashSet::new();
    bruteforce(&mut output, registers, &binary, &mut cache).unwrap()
}

fn bruteforce(
    output: &mut Vec<u8>,
    mut registers: [u64; 3],
    binary: &[u8],
    cache: &mut HashSet<u64>,
) -> Option<u64> {
    match run_program(binary, registers, output, true) {
        None => Some(registers[A]),
        Some(i) => {
            // REVERSE insight: every iteration look the 3 last bits + some random 3 bits before (worst case 7..10)
            let offset = (3 * i) as usize;
            let mask = !(((1 << 10) - 1) << offset);
            let max_a = 1 << (3 * binary.len());
            for local_a in 0..(1 << 10) {
                registers[A] &= mask;
                registers[A] |= local_a << offset;
                if registers[A] >= max_a {
                    break;
                }
                if cache.contains(&registers[A]) {
                    continue;
                }
                match run_program(binary, registers, output, true) {
                    None => {
                        return Some(registers[A]);
                    }
                    Some(i2) if i2 > i && i2 < binary.len() => {
                        let res = bruteforce(output, registers, binary, cache);
                        if res.is_some() {
                            return res;
                        }
                    }
                    _ => {}
                }
                cache.insert(registers[A]);
            }
            None
        }
    }
}

fn parse(input: &'static str) -> ([u64; 3], Vec<u8>) {
    let mut lines = input.trim().lines();
    let mut registers: [u64; 3] = [0; 3];
    for i in 0..3 {
        registers[i] = lines.next().unwrap()[12..].parse().unwrap();
    }
    let binary = lines.skip(1).next().unwrap()[9..]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u8>>();
    (registers, binary)
}

fn run_program(
    binary: &[u8],
    mut registers: [u64; 3],
    output: &mut Vec<u8>,
    shortcut: bool,
) -> Option<usize> {
    output.clear();
    let mut instr_ptr = 0;
    while instr_ptr < binary.len() {
        let instruction = binary[instr_ptr];
        let operand = binary[instr_ptr + 1];
        match instruction {
            ADV => {
                registers[A] /= 1 << combo(operand, &registers);
            }
            BXL => {
                registers[B] ^= operand as u64;
            }
            BST => {
                registers[B] = combo(operand, &registers) % 8;
            }
            JNZ => {
                if registers[A] != 0 {
                    instr_ptr = operand as usize;
                    continue;
                }
            }
            BXC => {
                registers[B] ^= registers[C];
            }
            OUT => {
                let new_out = (combo(operand, &registers) % 8) as u8;
                if shortcut && (output.len() >= binary.len() || binary[output.len()] != new_out) {
                    return Some(output.len());
                }
                output.push(new_out);
            }
            BDV => {
                registers[B] = registers[A] / (1 << combo(operand, &registers));
            }
            CDV => {
                registers[C] = registers[A] / (1 << combo(operand, &registers));
            }
            _ => panic!("invalid instruction {instruction}"),
        }
        instr_ptr += 2;
    }
    if output.len() == binary.len() {
        None
    } else {
        Some(output.len())
    }
}

fn combo(operand: u8, registers: &[u64; 3]) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4..7 => registers[(operand - 4) as usize],
        _ => panic!("invalid operand {operand}"),
    }
}
// registers
const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
// instructions
const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

#[test]
fn test1() {
    const EXAMPLE: &'static str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    assert_eq!(part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
}
#[test]
fn test2() {
    const EXAMPLE: &'static str = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    assert_eq!(part2(EXAMPLE), 117440);
}
