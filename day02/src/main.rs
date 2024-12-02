const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {}", answer);
    let answer = part2(INPUT);
    println!("Part 2 answer is {}", answer);
}

fn part1(input: &str) -> usize {
    parse(input)
        .filter(|numbers| get_unsafe_idx(&numbers).is_none())
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .filter_map(|mut numbers| {
            get_unsafe_idx(&numbers)
                .is_none_or(|i| {
                    let num = numbers.remove(i);
                    get_unsafe_idx(&numbers).is_none_or(|_| {
                        numbers[i] = num;
                        get_unsafe_idx(&numbers).is_none()
                    })
                })
                .then_some(1)
        })
        .count()
}

fn get_unsafe_idx(numbers: &[i32]) -> Option<usize> {
    let sign = (numbers[1] - numbers[0]).signum();
    if sign == 0 {
        return Some(0);
    }
    numbers
        .windows(2)
        .position(|t| !((1..=3).contains(&((t[1] - t[0]) * sign))))
}

fn parse(input: &str) -> impl Iterator<Item = Vec<i32>> + '_ {
    input.trim().lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}

#[cfg(test)]
const EXAMPLE: &'static str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

#[test]
fn test1() {
    assert_eq!(part1(EXAMPLE), 2);
}
#[test]
fn test2() {
    assert_eq!(part2(EXAMPLE), 4);
}
