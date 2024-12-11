use std::ops::{Add, Mul};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u64 {
    part_x(input, [Add::add, Mul::mul])
}
fn part2(input: &'static str) -> u64 {
    part_x(input, [Add::add, Mul::mul, concat])
}

fn concat(a: u64, b: u64) -> u64 {
    let mut power = 10;
    while b >= power {
        power *= 10;
    }
    a * power + b
}
fn part_x<const N: usize>(input: &'static str, ops: [fn(u64, u64) -> u64; N]) -> u64 {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (ans, terms) = line.split_once(':').unwrap();
            let ans: u64 = ans.parse().unwrap();
            let mut terms = terms
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .rev()
                .collect::<Vec<u64>>();
            let init = terms.pop().unwrap();
            let res = match_equation(ans, init, &mut terms, ops).then_some(ans);
            res
        })
        .sum()
}
fn match_equation<const N: usize>(
    ans: u64,
    acc: u64,
    terms: &mut Vec<u64>,
    ops: [fn(u64, u64) -> u64; N],
) -> bool {
    if acc > ans {
        return false;
    }
    if terms.is_empty() {
        return ans == acc;
    }
    let t = terms.pop().unwrap();
    for func in ops {
        if match_equation(ans, func(acc, t), terms, ops) {
            return true;
        }
    }
    terms.push(t);
    false
}

#[cfg(test)]
const EXAMPLE: &'static str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 3749);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 11387);
}
