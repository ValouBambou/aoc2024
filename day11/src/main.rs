use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u64 {
    part_x(input, 25)
}
fn part2(input: &'static str) -> u64 {
    part_x(input, 75)
}
fn part_x(input: &'static str, nblinks: u8) -> u64 {
    let mut table0 = HashMap::<u64, u64>::new();
    let mut table1 = HashMap::<u64, u64>::new();
    input
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .for_each(|i| {
            *table0.entry(i).or_insert(0) += 1;
        });
    let mut cur0 = true;
    for _i in 0..nblinks {
        let (src, dst) = if cur0 {
            (&table0, &mut table1)
        } else {
            (&table1, &mut table0)
        };
        blink(src, dst);
        cur0 = !cur0;
    }
    let last_step = if cur0 { table0 } else { table1 };
    last_step.into_values().sum()
}
fn blink(src: &HashMap<u64, u64>, dst: &mut HashMap<u64, u64>) {
    dst.clear();
    for (&x, &count) in src.iter() {
        let digits = ndigits(x);
        match x {
            0 => {
                *dst.entry(1).or_insert(0) += count;
            }
            x if digits % 2 == 0 => {
                let power = 10u64.pow(digits as u32 / 2);
                *dst.entry(x / power).or_insert(0) += count;
                *dst.entry(x % power).or_insert(0) += count;
            }
            x => {
                *dst.entry(x * 2024).or_insert(0) += count;
            }
        }
    }
}
fn ndigits(n: u64) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}
#[cfg(test)]
const EXAMPLE: &'static str = "125 17";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 55312);
}
