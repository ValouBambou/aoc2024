const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let answer = part1(INPUT);
    println!("Part 1 answer is {answer}");
    let answer = part2(INPUT);
    println!("Part 2 answer is {answer}");
}

fn part1(input: &str) -> u32 {
    input
        .split("mul(")
        .map(|s| parse_mul(s).unwrap_or_default())
        .sum()
}

fn parse_mul(s: &str) -> Option<u32> {
    let (n1, rest) = s.split_once(",")?;
    let n1: u32 = n1.parse().ok()?;
    let (n2, _) = rest.split_once(")")?;
    let n2: u32 = n2.parse().ok()?;
    debug_assert!(n1 < 1000);
    debug_assert!(n2 < 1000);
    Some(n1 * n2)
}
fn part2(input: &str) -> u32 {
    let bytes = input.as_bytes();
    let mut do_mul = true;
    let mut res = 0;
    for i in 0..(bytes.len() - 4) {
        if &bytes[i..(i + 4)] == b"do()" {
            do_mul = true;
        } else if i + 7 < bytes.len() && &bytes[i..(i + 7)] == b"don't()" {
            do_mul = false;
        } else if do_mul && &bytes[i..(i + 4)] == b"mul(" {
            let substring = input.split_at(i + 4).1;
            let mul = parse_mul(substring).unwrap_or_default();
            res += mul;
        }
    }
    res
}

#[test]
fn test1() {
    const EXAMPLE: &'static str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(EXAMPLE), 161);
}
#[test]
fn test2() {
    const EXAMPLE: &'static str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(EXAMPLE), 48);
}
