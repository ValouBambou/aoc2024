use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {}", answer);
    let answer = part2(INPUT);
    println!("Part 2 answer is {}", answer);
}

fn part1(input: &str) -> u32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.trim().lines() {
        let mut split = line.split("   ");
        let n1: u32 = split.next().unwrap().parse().unwrap();
        let n2: u32 = split.next().unwrap().parse().unwrap();
        list1.push(n1);
        list2.push(n2);
    }
    list1.sort();
    list2.sort();
    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(n1, n2)| n2.abs_diff(n1))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut occurences: HashMap<u32, u32> = HashMap::new();
    let list1: Vec<u32> = input
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.split("   ");
            let n1: u32 = split.next().unwrap().parse().unwrap();
            let n2: u32 = split.next().unwrap().parse().unwrap();
            *occurences.entry(n2).or_insert(0) += 1;
            n1
        })
        .collect();
    list1
        .into_iter()
        .map(|n1| n1 * *occurences.get(&n1).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
const EXAMPLE: &'static str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 11);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 31);
}
