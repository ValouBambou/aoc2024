use std::collections::{HashMap, HashSet};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> u64 {
    part_x(input, true)
}
fn part2(input: &'static str) -> u64 {
    part_x(input, false)
}
fn part_x(input: &'static str, part1: bool) -> u64 {
    let mut lines = input.trim().lines();
    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .collect::<HashSet<&'static str>>();
    let len_max_pattern = patterns.iter().map(|x| x.len()).max().unwrap();
    let mut lru = HashMap::new();
    lines.next().unwrap();
    lines
        .map(|x| {
            let res = made_with_patterns(x, &patterns, len_max_pattern, &mut lru);
            if part1 {
                (res > 0) as u64
            } else {
                res
            }
        })
        .sum()
}

fn made_with_patterns(
    x: &'static str,
    patterns: &HashSet<&'static str>,
    len_max_pattern: usize,
    lru: &mut HashMap<&'static str, u64>,
) -> u64 {
    if let Some(&v) = lru.get(x) {
        return v;
    }
    let mut total = patterns.contains(x) as u64;
    for i in 1..x.len().min(len_max_pattern + 1) {
        let (left, right) = x.split_at(i);
        if patterns.contains(left) {
            let n_right = made_with_patterns(right, patterns, len_max_pattern, lru);
            total += n_right;
        }
    }
    lru.insert(x, total);
    total
}
#[cfg(test)]
const EXAMPLE: &'static str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 6);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 16);
}
