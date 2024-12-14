use std::ops::{Add, Mul};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &'static str) -> i64 {
    part_x(input, 0)
}
fn part2(input: &'static str) -> i64 {
    part_x(input, 10000000000000)
}
fn part_x(input: &'static str, offset: i64) -> i64 {
    input
        .trim()
        .split("\n\n")
        .filter_map(|paragraph| {
            let mut coord = paragraph.lines().map(|line| {
                let (left, right) = line.split_once(", ").unwrap();
                let x = left.split_once("X").unwrap().1[1..].parse().unwrap();
                let y = right.split_once("Y").unwrap().1[1..].parse().unwrap();
                [x, y]
            });
            let coord_a = coord.next().unwrap();
            let coord_b = coord.next().unwrap();
            let target = coord.next().unwrap().map(|x| x + offset);
            min_cost(coord_a, coord_b, target)
        })
        .sum()
}

fn min_cost(coord_a: [i64; 2], coord_b: [i64; 2], target: [i64; 2]) -> Option<i64> {
    let [x_a, y_a] = coord_a;
    let [x_b, y_b] = coord_b;
    let [x, y] = target;
    let num_b = x_a * y - x * y_a;
    let denum_b = x_a * y_b - x_b * y_a;
    if num_b % denum_b != 0 {
        return None;
    }
    let b = num_b / denum_b;
    let num_a = x - x_b * b;
    if num_a % x_a != 0 {
        return None;
    }
    let a = num_a / x_a;
    Some(3 * a + b)
}

#[cfg(test)]
const EXAMPLE: &'static str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 480);
}
